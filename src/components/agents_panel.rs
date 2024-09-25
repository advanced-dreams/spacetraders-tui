use ratatui::{
    layout::{Constraint, Layout, Rect},
    prelude::*,
    style::{Color, Style},
    widgets::{Block, BorderType, List, ListDirection, ListState, Paragraph},
};

use crate::action::Action;
use crate::components::Component;
use crate::state::State;
use crate::tabs::Tabs;
use crate::tui::Frame;

const AGENTS: [&str; 4] = [
    "Get Agent",
    "List Agents",
    "Get Public Agent",
    "Register New Agent",
];

pub struct AgentsPanel {
    action_list: ListState,
    is_focused: bool,
}

impl Default for AgentsPanel {
    fn default() -> Self {
        Self {
            action_list: ListState::default(),
            is_focused: false,
        }
    }
}

impl Component for AgentsPanel {
    fn update(&mut self, action: Action, state: &State) -> color_eyre::Result<Option<Action>> {
        match action {
            Action::ListDown => self.action_list.select_next(),
            Action::ListUp => self.action_list.select_previous(),
            _ => {}
        }

        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect, state: &State) -> color_eyre::Result<()> {
        let v_chunks = Layout::new(
            Direction::Vertical,
            vec![
                Constraint::Length(3),
                Constraint::Min(5),
                Constraint::Length(3),
            ],
        )
        .split(area);
        let chunks =
            Layout::horizontal([Constraint::Length(30), Constraint::Min(0)]).split(v_chunks[1]);
        let list = List::new(AGENTS)
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
        f.render_stateful_widget(list, chunks[0], &mut self.action_list);
        f.render_widget(
            Paragraph::new(format!("Action output goes here."))
                .block(
                    Block::bordered()
                        .title("Action Results")
                        .title_alignment(Alignment::Left)
                        .border_type(BorderType::Rounded),
                )
                .style(Style::default().fg(Color::Cyan).bg(Color::Black))
                .centered(),
            chunks[1],
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
        *tab == Tabs::Agents
    }
}
