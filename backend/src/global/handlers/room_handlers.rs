use crate::global::room_manager::room_manager;
use crate::model::room::RoomSimpleInfo;
use crate::transport::request::RequestHandler;
use anyhow::Error;
use futures_util::future::BoxFuture;
use futures_util::FutureExt;
use std::ops::Deref;

pub struct ListRoomSimpleInfoHandler;

impl RequestHandler<(), Vec<RoomSimpleInfo>> for ListRoomSimpleInfoHandler {
    fn handle(&self, _: u32, _: ()) -> BoxFuture<Result<Vec<RoomSimpleInfo>, Error>> {
        async move {
            Ok(room_manager()
                .all()
                .iter()
                .map(|room| room.read().deref().into())
                .collect())
        }
        .boxed()
    }
}

pub struct CreateRoomHandler;

impl RequestHandler<(), ()> for CreateRoomHandler {
    fn handle(&self, uid: u32, _: ()) -> BoxFuture<Result<(), Error>> {
        async move { Ok(()) }.boxed()
    }
}
