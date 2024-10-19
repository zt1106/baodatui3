use crate::data_structure::shared_map::{GlobalMap, WithId};
use crate::model::user::User;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};

#[derive(Default)]
pub struct UserManager {
    uuid_map: Arc<RwLock<HashMap<String, Arc<RwLock<User>>>>>,
}

pub fn user_manager() -> &'static UserManager {
    static MANAGER: OnceLock<UserManager> = OnceLock::new();
    MANAGER.get_or_init(|| Default::default())
}

impl UserManager {
    fn id_map() -> &'static GlobalMap<User> {
        static ID_MAP: OnceLock<GlobalMap<User>> = OnceLock::new();
        ID_MAP.get_or_init(|| GlobalMap::default())
    }

    pub fn find_user_by_uuid(&self, uuid: &String) -> Option<Arc<RwLock<User>>> {
        self.uuid_map.read().get(uuid).cloned()
    }

    pub fn add(&self, t: User) -> Arc<RwLock<User>> {
        let result = Self::id_map().add(t);
        self.uuid_map
            .write()
            .insert(result.read().uuid.clone(), result.clone());
        result
    }

    pub fn add_default(&self) -> Arc<RwLock<User>> {
        Self::id_map().add_default()
    }

    pub fn remove_id(&self, id: u32) {
        let cur_opt = Self::id_map().get(id);
        match cur_opt {
            None => {
                return;
            }
            Some(cur) => {
                self.uuid_map.write().remove(&cur.read().uuid);
                Self::id_map().remove_id(id);
            }
        }
    }

    pub fn remove(&self, t: Arc<RwLock<User>>) {
        let id = t.read().id();
        self.remove_id(id);
    }

    pub fn get(&self, id: u32) -> Option<Arc<RwLock<User>>> {
        let map = Self::id_map();
        map.get(id).and_then(|t| Some(t.clone()))
    }

    pub fn contains_id(&self, id: u32) -> bool {
        Self::id_map().contains_id(id)
    }

    pub fn find_first(&self, f: impl Fn(&User) -> bool) -> Option<Arc<RwLock<User>>> {
        Self::id_map().find_first(f)
    }

    pub fn all(&self) -> Vec<Arc<RwLock<User>>> {
        Self::id_map().all()
    }
}
