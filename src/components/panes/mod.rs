use crate::components::{Action, Frame, KeyEvent};
use crate::tui::Event;
use color_eyre::eyre::Result;
use ratatui::layout::{Constraint, Rect};

pub mod response_viewer;

// TODO: trait may need further tweaking since it's lifted from Astray
// TODO: come back to this later after getting a basic response working

// pub trait Pane {
//     fn init(&mut self, _state: &State) -> Result<()> {
//         Ok(())
//     }
//
//     fn height_constraint(&self) -> Constraint;
//
//     fn handle_events(&mut self, event: Event, state: &mut State) -> Result<Option<Action>> {
//         let r = match event {
//             Event::Key(key_event) => self.handle_key_events(key_event, state)?,
//             _ => None,
//         };
//         Ok(r)
//     }
//
//     fn handle_key_events(&mut self, _key: KeyEvent, _state: &mut State) -> Result<Option<Action>> {
//         Ok(None)
//     }
//
//     fn update(&mut self, _action: Action, _state: &mut State) -> Result<Option<Action>> {
//         Ok(None)
//     }
//
//     fn draw(&mut self, f: &mut Frame<'_>, area: Rect, state: &State) -> Result<()>;
// }
