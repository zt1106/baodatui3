use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::container::user_manager::user_manager;

use super::Handler;

#[derive(Serialize, Deserialize)]
pub struct ChangeUserNameRequestBody {
    pub new_name: String,
}
pub struct ChangeUserNameHandler;

#[async_trait]
impl Handler<ChangeUserNameRequestBody, ()> for ChangeUserNameHandler {
    async fn handle(
        &self,
        request_body: ChangeUserNameRequestBody,
        uid: u32,
    ) -> super::Response<()> {
        return if let Some(user) = user_manager().get(uid) {
            let mut user = user.lock().await;
            user.nick_name = request_body.new_name;
            super::Response::Data(())
            // TODO should also trigger room refresh event
        } else {
            super::Response::Error("user not found".to_string())
        };
    }
}
