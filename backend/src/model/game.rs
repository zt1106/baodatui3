use std::sync::Arc;
use tokio::sync::RwLock;
use crate::model::configurable_rules::ConfigurableRules;
use crate::model::user::User;

pub struct Game {
    players: Vec<Player>,
    configurable_rules: ConfigurableRules,
}

pub struct Player {
    user: Arc<RwLock<User>>,
}