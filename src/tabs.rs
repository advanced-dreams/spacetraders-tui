use serde::{Deserialize, Serialize};
use strum::{Display, VariantArray};

#[derive(
    Clone, Copy, Display, Default, Eq, PartialEq, Serialize, Deserialize, Debug, VariantArray,
)]
pub enum Tabs {
    #[default]
    Fleet,
    Systems,
    Contracts,
    Agents,
    Factions,
    #[strum(serialize = "Game Status")]
    GameStatus,
}

// TODO: not sure if I need this
impl From<Tabs> for String {
    fn from(value: Tabs) -> Self {
        match value {
            Tabs::Fleet => String::from("Fleet"),
            Tabs::Systems => String::from("Systems"),
            Tabs::Contracts => String::from("Contracts"),
            Tabs::Agents => String::from("Agents"),
            Tabs::Factions => String::from("Factions"),
            Tabs::GameStatus => String::from("Game Status"),
        }
    }
}
