use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::container::user_manager::user_manager;

use super::Handler;

#[derive(Serialize, Deserialize)]
pub struct ChangeUserNameRequest {
    pub new_name: String,
}
#[derive(Serialize, Deserialize)]
pub struct ChangeUserNameResponse;

pub struct ChangeUserNameHandler;

#[async_trait]
impl Handler<ChangeUserNameRequest, ChangeUserNameResponse> for ChangeUserNameHandler {
    async fn handle(
        &self,
        request: ChangeUserNameRequest,
        uid: u32,
    ) -> super::Response<ChangeUserNameResponse> {
        return if let Some(user) = user_manager().get(uid) {
            let mut user = user.lock().await;
            user.nick_name = request.new_name;
            super::Response::Data(ChangeUserNameResponse)
            // TODO should also trigger room refresh event
        } else {
            super::Response::Error("user not found".to_string())
        }
    }
}
