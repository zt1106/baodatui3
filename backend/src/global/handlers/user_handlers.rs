use crate::global::user_manager::user_manager;
use crate::model::user::User;
use crate::transport::request::{RequestHandler, RequestType};
use anyhow::Error;
use futures_util::future::BoxFuture;
use futures_util::FutureExt;

pub struct GetCurUserHandler;

pub const GET_CUR_USER_REQ_TYPE: RequestType<(), User> = RequestType::new("GetCurUser");

impl RequestHandler<(), User> for GetCurUserHandler {
    fn handle(&self, uid: u32, _req: ()) -> BoxFuture<Result<User, Error>> {
        async move {
            user_manager()
                .get(uid)
                .map(|u| u.read().clone())
                .ok_or(Error::msg("User not found"))
        }
        .boxed()
    }
}

pub struct ChangeCurUserNameHandler;

pub const CHANGE_CUR_USER_NAME_REQ_TYPE: RequestType<String, ()> =
    RequestType::new("ChangeCurUserName");

impl RequestHandler<String, ()> for ChangeCurUserNameHandler {
    fn handle(&self, uid: u32, req: String) -> BoxFuture<Result<(), Error>> {
        async move {
            if req.is_empty() {
                return Err(Error::msg("Empty user name request"));
            }
            if req.len() > 10 {
                return Err(Error::msg("name is too long"));
            }
            let user = user_manager()
                .get(uid)
                .ok_or(Error::msg("User not found"))?;
            let mut user_lock = user.write();
            user_lock.nick_name = req;
            Ok(())
        }
        .boxed()
    }
}
