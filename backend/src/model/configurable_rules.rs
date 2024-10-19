use serde::{Deserialize, Serialize};

/// all configurable rules of how the game is played
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ConfigurableRules {
    basic_rules: BasicRules,
    play_rules: PlayRules,
    time_rules: TimeRules,
    score_rules: ScoreRules,
}
#[derive(Debug, Deserialize, Serialize, Clone)]

pub struct BasicRules {
    player_count: u8,
    deck_size: u8,
}
#[derive(Debug, Deserialize, Serialize, Clone)]

pub struct PlayRules {}
#[derive(Debug, Deserialize, Serialize, Clone)]

pub struct TimeRules {}
#[derive(Debug, Deserialize, Serialize, Clone)]

pub struct ScoreRules {}
