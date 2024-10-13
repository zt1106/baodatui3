/// all configurable rules of how the game is played
pub struct ConfigurableRules {
    basic_rules: BasicRules,
    play_rules: PlayRules,
    time_rules: TimeRules,
    score_rules: ScoreRules,
}

pub struct BasicRules {
    player_count: u8,
    deck_size: u8,
}

pub struct PlayRules {}

pub struct TimeRules {}

pub struct ScoreRules {}