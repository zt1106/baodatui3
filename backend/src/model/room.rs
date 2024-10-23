use crate::model::configs::GameConfigurations;
use crate::model::game::Game;
use crate::model::user::User;
use crate::utils::WatcherWrapper;
use baodatui_macro::ID;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::Arc;

#[derive(Serialize, Deserialize, Clone)]
pub enum RoomStatus {
    Waiting,
    InGame,
}

impl Default for RoomStatus {
    fn default() -> Self {
        RoomStatus::Waiting
    }
}

/// owner of the room is the first user in the room
#[derive(ID, Default)]
pub struct Room {
    pub id: u32,
    pub users: Vec<Arc<RwLock<User>>>,
    game_configs: GameConfigurations,
    pub cur_game: Option<Game>,
    pub status: RoomStatus,
    pub detailed_info_change_watch: WatcherWrapper<RoomDetailedInfo>,
}

impl Room {
    pub fn owner(&self) -> Arc<RwLock<User>> {
        self.users[0].clone()
    }

    pub fn update_users(&mut self, f: impl FnOnce(&mut Vec<Arc<RwLock<User>>>) -> ()) {
        f(&mut self.users);
        self.notify_detail_changed();
    }

    pub fn notify_detail_changed(&mut self) {
        self.detailed_info_change_watch.send(self.deref().into())
    }

    pub fn game_configs(&self) -> &GameConfigurations {
        &self.game_configs
    }

    pub fn update_game_configs(&mut self, configs: GameConfigurations) {
        self.game_configs = configs;
        self.notify_detail_changed();
    }
}

// information needed to be displayed in lobby
#[derive(Serialize, Deserialize, Clone)]
pub struct RoomSimpleInfo {
    pub id: u32,
    pub status: RoomStatus,
    pub cur_user_count: usize,
    pub max_user_count: usize,
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

// information needed to render the room page
#[derive(Serialize, Deserialize, Clone)]
pub struct RoomDetailedInfo {
    pub id: u32,
    pub status: RoomStatus,
    pub user_in_room_infos: Vec<UserInRoomInfo>,
    pub config: GameConfigurations,
}

// user information needed to render the room page
#[derive(Serialize, Deserialize, Clone)]
pub struct UserInRoomInfo {
    pub prepared: bool,
    pub nick_name: String,
}

impl From<&User> for UserInRoomInfo {
    fn from(value: &User) -> Self {
        Self {
            prepared: value.prepared,
            nick_name: value.nick_name.clone(),
        }
    }
}

impl From<&Room> for RoomDetailedInfo {
    fn from(value: &Room) -> Self {
        Self {
            id: value.id,
            status: value.status.clone(),
            config: value.game_configs.clone(),
            user_in_room_infos: value
                .users
                .iter()
                .map(|u| u.read().deref().into())
                .collect(),
        }
    }
}
