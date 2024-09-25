use ratatui::{
    layout::{Constraint, Layout, Rect},
    prelude::*,
    style::{Color, Style},
    widgets::{Block, BorderType, List, ListDirection, ListState, Paragraph, Wrap},
};
use reqwest::{Method, Request, Url};
use std::str::FromStr;
use tokio::sync::mpsc::UnboundedSender;

use crate::action::Action;
use crate::components::Component;
use crate::state::State;
use crate::tabs::Tabs;
use crate::tui::Frame;

const GAME_STATUS: [&str; 2] = ["Get Status", "Register New Agent"];

pub struct GameStatusPanel {
    action_list: ListState,
    command_tx: Option<UnboundedSender<Action>>,
    request_tx: Option<UnboundedSender<Request>>,
    is_focused: bool,
}

impl Default for GameStatusPanel {
    fn default() -> Self {
        Self {
            // TODO: consider renaming 'action_list'
            action_list: ListState::default(),
            command_tx: None,
            request_tx: None,
            is_focused: false,
        }
    }
}

impl Component for GameStatusPanel {
    // TODO: add this function to other panel components
    fn register_action_handler(
        &mut self,
        tx: UnboundedSender<Action>,
    ) -> color_eyre::eyre::Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    // TODO: add this function to other panel components
    fn register_request_handler(
        &mut self,
        tx: UnboundedSender<Request>,
    ) -> color_eyre::eyre::Result<()> {
        self.request_tx = Some(tx);
        Ok(())
    }

    fn update(&mut self, action: Action, state: &State) -> color_eyre::Result<Option<Action>> {
        match action {
            Action::ListDown => self.action_list.select_next(),
            Action::ListUp => self.action_list.select_previous(),
            Action::SelectItem
                if GAME_STATUS[self.action_list.selected().unwrap()] == "Get Status" =>
            {
                // TODO: update to use RequestBuilder
                let status_request = reqwest::Request::new(
                    Method::GET,
                    Url::from_str("https://api.spacetraders.io/v2/").unwrap(),
                );
                if let Some(request_tx) = &self.request_tx {
                    request_tx.send(status_request)?;
                };
            }
            Action::SelectItem
                if GAME_STATUS[self.action_list.selected().unwrap()] == "Register New Agent" =>
            {
                todo!();
            }
            _ => {}
        }

        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect, state: &State) -> color_eyre::Result<()> {
        let v_chunks = Layout::new(
            Direction::Vertical,
            vec![
                Constraint::Length(3), // top menu
                Constraint::Min(5),    // action list and request/response panel
                Constraint::Length(3), // footer
            ],
        )
        .split(area);
        let h_chunks =
            Layout::horizontal([Constraint::Length(30), Constraint::Min(0)]).split(v_chunks[1]);
        let request_chunks = Layout::new(
            Direction::Vertical,
            vec![
                Constraint::Percentage(25), // request body
                Constraint::Percentage(75), // response
            ],
        )
        .split(h_chunks[1]);
        let list = List::new(GAME_STATUS)
            .block(
                Block::bordered()
                    .title("Actions")
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Cyan).bg(Color::Black))
            .highlight_style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">")
            .highlight_spacing(ratatui::widgets::HighlightSpacing::Always)
            .repeat_highlight_symbol(true)
            .direction(ListDirection::TopToBottom);
        f.render_stateful_widget(list, h_chunks[0], &mut self.action_list);
        f.render_widget(
            Paragraph::new(Text::raw("Request body."))
                .wrap(Wrap { trim: false })
                .block(
                    Block::bordered()
                        .title("Request")
                        .title_alignment(Alignment::Left)
                        .border_type(BorderType::Rounded),
                )
                .style(Style::default().fg(Color::Cyan).bg(Color::Black))
                .left_aligned(),
            request_chunks[0],
        );
        f.render_widget(
            Paragraph::new(Text::raw(&state.responses))
                .wrap(Wrap { trim: false })
                .block(
                    Block::bordered()
                        .title("Response")
                        .title_alignment(Alignment::Left)
                        .border_type(BorderType::Rounded),
                )
                .style(Style::default().fg(Color::Cyan).bg(Color::Black))
                .left_aligned(),
            request_chunks[1],
        );
        f.render_widget(
            Paragraph::new(format!("Extra context / help / keybindings go here."))
                .block(
                    Block::bordered()
                        .title("Help Footer")
                        .title_alignment(Alignment::Left)
                        .border_type(BorderType::Rounded),
                )
                .style(Style::default().fg(Color::Cyan).bg(Color::Black))
                .centered(),
            v_chunks[2],
        );

        Ok(())
    }

    fn is_drawn_in_tab(&self, tab: &Tabs) -> bool {
        *tab == Tabs::GameStatus
    }
}
