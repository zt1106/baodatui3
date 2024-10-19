use crate::model::user::User;
use parking_lot::RwLock;
use std::sync::{Arc, OnceLock};

use crate::data_structure::shared_map::GlobalMap;

pub fn user_manager() -> &'static GlobalMap<User> {
    static USER_MANAGER: OnceLock<GlobalMap<User>> = OnceLock::new();
    USER_MANAGER.get_or_init(|| GlobalMap::default())
}

impl GlobalMap<User> {
    pub fn find_user_by_uuid(&self, uuid: &str) -> Option<Arc<RwLock<User>>> {
        self.find(|user| user.uuid == uuid)
    }
}
