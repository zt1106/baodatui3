use crate::ext::AsyncMap;
use crate::global::user_manager::user_manager;
use crate::model::user::User;
use crate::transport::request::RequestHandler;
use anyhow::Error;
use futures_util::future::BoxFuture;
use futures_util::FutureExt;

pub struct GetCurUserHandler;

impl RequestHandler<(), User> for GetCurUserHandler {
    fn handle(&self, uid: u32, _req: ()) -> BoxFuture<Result<User, Error>> {
        async move {
            user_manager().get(uid).async_map(|u| async move { u.read().await.clone() }.boxed()).await.ok_or(Error::msg("User not found"))
        }.boxed()
    }
}