use crate::model::game::Game;
use crate::model::user::User;
use parking_lot::RwLock;
use std::sync::Arc;

pub struct Room {
    users: Vec<Arc<RwLock<User>>>,
    cur_game: Option<Game>,
}
