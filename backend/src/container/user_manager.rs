use std::sync::{Arc, OnceLock};

use tokio::sync::Mutex;
use uuid::Uuid;

use crate::model::user::{create_random_chinese_name, User};

use super::arcmap::ArcMap;

#[allow(dead_code)]
static USER_MANAGER: OnceLock<ArcMap<User>> = OnceLock::new();

pub fn user_manager() -> &'static ArcMap<User> {
    USER_MANAGER.get_or_init(|| ArcMap::default())
}

impl ArcMap<User> {
    pub async fn add_user(&self) -> Arc<Mutex<User>> {
        let mut user = User::default();
        user.nick_name = create_random_chinese_name();
        user.uuid = Uuid::new_v4().to_string();
        self.add(user)
    }

    pub async fn find_user_by_uuid(&self, uuid: &str) -> Option<Arc<Mutex<User>>> {
        self.find(|user| user.uuid == uuid).await
    }
}
