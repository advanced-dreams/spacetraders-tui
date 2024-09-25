use serde::{Deserialize, Serialize};
use strum::Display;

use crate::tabs::Tabs;

#[derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize)]
pub enum Action {
    Tick,
    Render,
    Resize(u16, u16),
    Suspend,
    Resume,
    Quit,
    ClearScreen,
    Error(String),
    Help,

    // Loading data
    LoadTabs(Vec<Tabs>),

    // Navigation
    NavigateNextTab,
    NavigatePrevTab,

    // List actions
    ListDown,
    ListUp,
    SelectItem,

    // HTTP requests
    Dial,
}
