use crate::model::configurable_rules::GameConfigurations;
use crate::model::game::Game;
use crate::model::user::User;
use baodatui_macro::ID;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::sync::Arc;

#[derive(Serialize, Deserialize, Clone)]
pub enum RoomStatus {
    A,
}

impl Default for RoomStatus {
    fn default() -> Self {
        RoomStatus::A
    }
}

#[derive(ID, Default)]
pub struct Room {
    pub id: u32,
    pub users: Vec<Arc<RwLock<User>>>,
    game_configs: GameConfigurations,
    cur_game: Option<Game>,
    status: RoomStatus,
}

// information needed to be displayed in lobby
#[derive(Serialize, Deserialize)]
pub struct RoomSimpleInfo {
    id: u32,
    status: RoomStatus,
    cur_user_count: usize,
    max_user_count: usize,
}

impl From<&Room> for RoomSimpleInfo {
    fn from(value: &Room) -> Self {
        let cur_user_count: usize = value.users.len();
        let max_user_count = value.game_configs.basic_configs.max_player_count as usize;
        Self {
            id: value.id,
            status: value.status.clone(),
            cur_user_count,
            max_user_count,
        }
    }
}
