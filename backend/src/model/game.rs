use crate::model::configurable_rules::GameConfigurations;
use crate::model::user::User;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct Game {
    players: Vec<Player>,
    configurable_rules: GameConfigurations,
}

pub struct Player {
    user: Arc<RwLock<User>>,
}
