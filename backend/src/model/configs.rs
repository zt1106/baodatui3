use serde::{Deserialize, Serialize};

/// all configurable rules of how the game is played
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct GameConfigurations {
    pub basic_configs: BasicConfigurations,
    pub play_configs: PlayConfigurations,
    pub time_configs: TimeConfigurations,
    pub score_configs: ScoreConfigurations,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BasicConfigurations {
    pub max_player_count: u8,
    pub deck_size: u8,
}

impl Default for BasicConfigurations {
    fn default() -> Self {
        Self {
            max_player_count: 6,
            deck_size: 4,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct PlayConfigurations {}
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct TimeConfigurations {}
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ScoreConfigurations {}
