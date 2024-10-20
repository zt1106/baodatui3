use crate::data_structure::shared_map::GlobalMap;
use crate::global::user_manager::user_manager;
use crate::model::configs::GameConfigurations;
use crate::model::room::Room;
use anyhow::{anyhow, Error};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};

pub fn room_manager() -> &'static RoomManager {
    static ROOM_MANAGER: OnceLock<RoomManager> = OnceLock::new();
    ROOM_MANAGER.get_or_init(|| RoomManager::default())
}

#[derive(Default)]
pub struct RoomManager {
    user_id_map: Arc<RwLock<HashMap<u32, Arc<RwLock<Room>>>>>,
}

impl RoomManager {
    fn id_map() -> &'static GlobalMap<Room> {
        static ROOM_MANAGER: OnceLock<GlobalMap<Room>> = OnceLock::new();
        ROOM_MANAGER.get_or_init(|| GlobalMap::default())
    }

    pub fn create_room_by_user_id(&self, user_id: u32) -> Result<Arc<RwLock<Room>>, Error> {
        if self.user_id_map.read().contains_key(&user_id) {
            return Err(anyhow!("User already in a room"));
        }
        let room = Self::id_map().add_default();
        room.write().users.push(
            user_manager()
                .get(user_id)
                .ok_or(anyhow!("User not found"))?,
        );
        self.user_id_map.write().insert(user_id, room.clone());
        Ok(room)
    }

    pub fn add_user_to_room(&self, user_id: u32, room_id: u32) -> Result<(), Error> {
        if self.user_id_map.read().contains_key(&user_id) {
            return Err(anyhow!("User already in a room"));
        }
        // TODO some state of user should be reset when enter a new room
        let room = Self::id_map()
            .get(room_id)
            .ok_or(anyhow!("Room not found {}", room_id))?;
        room.write().users.push(
            user_manager()
                .get(user_id)
                .ok_or(anyhow!("User not found {}", user_id))?,
        );
        self.user_id_map.write().insert(user_id, room.clone());
        Ok(())
    }

    pub fn remove_user_from_room(&self, user_id: u32, room_id: u32) -> Result<(), Error> {
        if !self.user_id_map.read().contains_key(&user_id) {
            return Err(anyhow!("User not in a room"));
        }
        let room = Self::id_map()
            .get(room_id)
            .ok_or(anyhow!("Room not found"))?;
        // TODO cannot remove when prepared, and in game
        room.write().users.retain(|u| u.read().id != user_id);
        self.user_id_map.write().remove(&user_id);
        if room.read().users.is_empty() {
            // when last person leave, remove room (room must have at least one user)
            self.remove_room(room.read().id);
        }
        Ok(())
    }

    pub fn find_room_by_user_id(&self, user_id: u32) -> Option<Arc<RwLock<Room>>> {
        self.user_id_map.read().get(&user_id).cloned()
    }

    pub fn remove_room(&self, room_id: u32) {
        Self::id_map().remove_id(room_id);
        self.user_id_map
            .write()
            .retain(|_, v| v.read().id != room_id);
    }

    pub fn all(&self) -> Vec<Arc<RwLock<Room>>> {
        Self::id_map().all()
    }

    pub fn update_game_configs_of_room(
        &self,
        room_id: u32,
        configs: GameConfigurations,
    ) -> Result<(), Error> {
        let room = Self::id_map()
            .get(room_id)
            .ok_or(anyhow!("Room not found {}", room_id))?;
        room.write().game_configs = configs;
        Ok(())
    }
}
