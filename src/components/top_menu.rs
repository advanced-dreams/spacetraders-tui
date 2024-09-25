use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Tabs},
};

use crate::action::Action;
use crate::components::Component;
use crate::state::State;
use crate::tui::Frame;

#[derive(Default)]
pub struct TopMenu {
    tabs: Vec<String>,
    index: usize,
}

impl Component for TopMenu {
    fn update(&mut self, action: Action, state: &State) -> color_eyre::Result<Option<Action>> {
        match action {
            Action::LoadTabs(tabs) => {
                self.tabs = tabs.iter().map(|t| String::from(t.clone())).collect();
            }
            Action::NavigateNextTab => {
                self.index = (self.index + 1) % self.tabs.len();
            }
            Action::NavigatePrevTab => {
                if self.index > 0 {
                    self.index -= 1;
                } else {
                    self.index = self.tabs.len() - 1;
                }
            }
            _ => {}
        }

        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect, state: &State) -> color_eyre::Result<()> {
        let chunks = Layout::new(
            Direction::Vertical,
            vec![Constraint::Length(3), Constraint::Min(0)],
        )
        .split(area);

        let tabs = Tabs::new(self.tabs.clone())
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .title("S P A C E // T R A D E R S"),
            )
            .style(Style::default().fg(Color::Cyan).bg(Color::Black))
            .highlight_style(Style::default().fg(Color::Yellow))
            .select(self.index);

        f.render_widget(tabs, chunks[0]);

        Ok(())
    }

    fn is_drawn_in_tab(&self, tab: &crate::tabs::Tabs) -> bool {
        true
    }
}
