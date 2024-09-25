use color_eyre::Result;
use crossterm::event::KeyEvent;
use ratatui::prelude::Rect;
use reqwest::Request;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::mpsc;
use tracing::{debug, info};

use crate::{
    action::Action,
    components::{
        agents_panel::AgentsPanel,
        contracts_panel::ContractsPanel,
        factions_panel::{self, FactionsPanel},
        fleet_panel::FleetPanel,
        gamestatus_panel::GameStatusPanel,
        systems_panel::{self, SystemsPanel},
        top_menu::TopMenu,
        Component,
    },
    config::Config,
    response::Response,
    state::State,
    tabs::Tabs as MenuTabs,
    tui::{Event, Tui},
};

/// Application
pub struct App {
    config: Config,
    tick_rate: f64,
    frame_rate: f64,
    components: Vec<Box<dyn Component>>,
    tabs: Vec<MenuTabs>,
    active_tab: usize,
    should_quit: bool,
    should_suspend: bool,
    mode: Mode,
    last_tick_key_events: Vec<KeyEvent>,
    action_tx: mpsc::UnboundedSender<Action>,
    action_rx: mpsc::UnboundedReceiver<Action>,
    request_tx: mpsc::UnboundedSender<Request>,
    request_rx: mpsc::UnboundedReceiver<Request>,
    state: State,
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Mode {
    #[default]
    Main,
}

impl App {
    pub fn new(tick_rate: f64, frame_rate: f64) -> Result<Self> {
        let (action_tx, action_rx) = mpsc::unbounded_channel::<Action>();
        let (request_tx, request_rx) = mpsc::unbounded_channel::<Request>();
        let top_menu = TopMenu::default();
        let fleet_panel = FleetPanel::default();
        let systems_panel = SystemsPanel::default();
        let contracts_panel = ContractsPanel::default();
        let agents_panel = AgentsPanel::default();
        let factions_panel = FactionsPanel::default();
        let gamestatus_panel = GameStatusPanel::default();
        Ok(Self {
            tick_rate,
            frame_rate,
            components: vec![
                Box::new(top_menu),
                Box::new(fleet_panel),
                Box::new(systems_panel),
                Box::new(contracts_panel),
                Box::new(agents_panel),
                Box::new(factions_panel),
                Box::new(gamestatus_panel),
            ],
            tabs: vec![
                MenuTabs::Fleet,
                MenuTabs::Systems,
                MenuTabs::Contracts,
                MenuTabs::Agents,
                MenuTabs::Factions,
                MenuTabs::GameStatus,
            ],
            active_tab: 0,
            should_quit: false,
            should_suspend: false,
            config: Config::new()?,
            mode: Mode::Main,
            last_tick_key_events: Vec::new(),
            action_tx,
            action_rx,
            request_tx,
            request_rx,
            state: State::default(),
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        self.action_tx.send(Action::LoadTabs(self.tabs.clone()))?;

        let mut tui = Tui::new()?
            // .mouse(true) // uncomment this line to enable mouse support
            .tick_rate(self.tick_rate)
            .frame_rate(self.frame_rate);
        tui.enter()?;

        for component in self.components.iter_mut() {
            component.register_action_handler(self.action_tx.clone())?;
        }
        for component in self.components.iter_mut() {
            component.register_request_handler(self.request_tx.clone())?;
        }
        for component in self.components.iter_mut() {
            component.register_config_handler(self.config.clone())?;
        }
        for component in self.components.iter_mut() {
            component.init(tui.size()?)?;
        }

        let action_tx = self.action_tx.clone();
        loop {
            self.handle_events(&mut tui).await?;
            self.handle_actions(&mut tui)?;

            while let Ok(request) = self.request_rx.try_recv() {
                if let Ok(response) = reqwest::Client::new().execute(request).await {
                    let body = Response {
                        status: response.status(),
                        version: response.version(),
                        headers: response.headers().clone(),
                        content_length: response.content_length(),
                        body: response.text().await?.clone(),
                    };
                    // TODO: for now, update a simple text string
                    let json: Value = serde_json::from_str(&body.body)?;
                    self.state.responses = format!(
                        "Status: {}\nReset Date: {}\nStats: {}\nNext Reset: {}",
                        json["status"],
                        json["resetDate"],
                        json["stats"],
                        json["serverResets"]["next"]
                    )
                }
            }

            if self.should_suspend {
                tui.suspend()?;
                action_tx.send(Action::Resume)?;
                action_tx.send(Action::ClearScreen)?;
                // tui.mouse(true);
                tui.enter()?;
            } else if self.should_quit {
                tui.stop()?;
                break;
            }
        }
        tui.exit()?;
        Ok(())
    }

    async fn handle_events(&mut self, tui: &mut Tui) -> Result<()> {
        let Some(event) = tui.next_event().await else {
            return Ok(());
        };
        let action_tx = self.action_tx.clone();
        match event {
            Event::Quit => action_tx.send(Action::Quit)?,
            Event::Tick => action_tx.send(Action::Tick)?,
            Event::Render => action_tx.send(Action::Render)?,
            Event::Resize(x, y) => action_tx.send(Action::Resize(x, y))?,
            Event::Key(key) => self.handle_key_event(key)?,
            _ => {}
        }
        for component in self.components.iter_mut() {
            if let Some(action) = component.handle_events(Some(event.clone()))? {
                action_tx.send(action)?;
            }
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<()> {
        let action_tx = self.action_tx.clone();
        let Some(keymap) = self.config.keybindings.get(&self.mode) else {
            return Ok(());
        };
        match keymap.get(&vec![key]) {
            Some(action) => {
                info!("Got action: {action:?}");
                action_tx.send(action.clone())?;
            }
            _ => {
                // If the key was not handled as a single key action,
                // then consider it for multi-key combinations.
                self.last_tick_key_events.push(key);

                // Check for multi-key combinations
                if let Some(action) = keymap.get(&self.last_tick_key_events) {
                    info!("Got action: {action:?}");
                    action_tx.send(action.clone())?;
                }
            }
        }
        Ok(())
    }

    fn handle_actions(&mut self, tui: &mut Tui) -> Result<()> {
        while let Ok(action) = self.action_rx.try_recv() {
            if action != Action::Tick && action != Action::Render {
                debug!("{action:?}");
            }
            match action {
                Action::Tick => {
                    self.last_tick_key_events.drain(..);
                }
                Action::Quit => self.should_quit = true,
                Action::Suspend => self.should_suspend = true,
                Action::Resume => self.should_suspend = false,
                Action::NavigateNextTab => {
                    self.active_tab += 1;
                    self.active_tab %= self.tabs.len();
                }
                Action::NavigatePrevTab => {
                    if self.active_tab != 0 {
                        self.active_tab -= 1;
                    } else {
                        self.active_tab = self.tabs.len() - 1;
                    }
                }
                Action::ClearScreen => tui.terminal.clear()?,
                Action::Resize(w, h) => self.handle_resize(tui, w, h)?,
                Action::Render => self.render(tui)?,
                _ => {}
            }
            for component in self.components.iter_mut() {
                if let Some(action) = component.update(action.clone(), &mut self.state)? {
                    self.action_tx.send(action)?
                };
            }
        }
        Ok(())
    }

    fn handle_resize(&mut self, tui: &mut Tui, w: u16, h: u16) -> Result<()> {
        tui.resize(Rect::new(0, 0, w, h))?;
        self.render(tui)?;
        Ok(())
    }

    fn render(&mut self, tui: &mut Tui) -> Result<()> {
        tui.draw(|frame| {
            for component in self.components.iter_mut() {
                if component.is_drawn_in_tab(&self.tabs[self.active_tab]) {
                    if let Err(err) = component.draw(frame, frame.area(), &mut self.state) {
                        let _ = self
                            .action_tx
                            .send(Action::Error(format!("Failed to draw: {:?}", err)));
                    }
                }
            }
        })?;
        Ok(())
    }
}
