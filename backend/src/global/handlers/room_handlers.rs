use crate::global::room_manager::room_manager;
use crate::model::configs::GameConfigurations;
use crate::model::room::{RoomDetailedInfo, RoomSimpleInfo};
use crate::transport::request::{RequestHandler, RequestType};
use crate::transport::stream::StreamHandler;
use anyhow::{anyhow, Error};
use futures::Stream;
use futures_util::future::BoxFuture;
use futures_util::FutureExt;
use std::ops::Deref;
use std::pin::Pin;
use tokio::spawn;

pub struct ListRoomSimpleInfoHandler;

pub const LIST_ROOM_SIMPLE_INFO_REQ_TYPE: RequestType<(), Vec<RoomSimpleInfo>> =
    RequestType::new("ListRoomSimpleInfo");

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

pub const CREATE_ROOM_REQ_TYPE: RequestType<(), ()> = RequestType::new("CreateRoom");

impl RequestHandler<(), ()> for CreateRoomHandler {
    fn handle(&self, uid: u32, _: ()) -> BoxFuture<Result<(), Error>> {
        async move {
            room_manager().create_room_by_user_id(uid)?;
            Ok(())
        }
        .boxed()
    }
}

pub struct LeaveRoomHandler;

pub const LEAVE_ROOM_REQ_TYPE: RequestType<(), ()> = RequestType::new("LeaveRoom");

impl RequestHandler<(), ()> for LeaveRoomHandler {
    fn handle(&self, uid: u32, _: ()) -> BoxFuture<Result<(), Error>> {
        async move {
            let room_id = room_manager()
                .find_room_by_user_id(uid)
                .ok_or(anyhow!("user not in room"))?
                .read()
                .id;
            room_manager().remove_user_from_room(uid, room_id)?;
            Ok(())
        }
        .boxed()
    }
}

pub struct EnterRoomHandler;

pub const ENTER_ROOM_REQ_TYPE: RequestType<u32, ()> = RequestType::new("EnterRoom");

impl RequestHandler<u32, ()> for EnterRoomHandler {
    fn handle(&self, uid: u32, req: u32) -> BoxFuture<Result<(), Error>> {
        async move { room_manager().add_user_to_room(uid, req) }.boxed()
    }
}

pub struct ChangeGameConfigHandler;

pub const CHANGE_GAME_CONFIG_REQ_TYPE: RequestType<GameConfigurations, ()> =
    RequestType::new("ChangeGameConfig");

impl RequestHandler<GameConfigurations, ()> for ChangeGameConfigHandler {
    fn handle(&self, uid: u32, req: GameConfigurations) -> BoxFuture<Result<(), Error>> {
        async move {
            let room = room_manager()
                .find_room_by_user_id(uid)
                .ok_or(anyhow!("user not in room"))?;
            if room.read().owner().read().id != uid {
                return Err(anyhow!("user is not owner"));
            }
            let room_id = room.read().id;
            room_manager().update_game_configs_of_room(room_id, req)?;
            Ok(())
        }
        .boxed()
    }
}

pub struct RoomDetailedInfoStreamHandler;

pub const ROOM_DETAILED_INFO_STREAM_TYPE: RequestType<(), RoomDetailedInfo> =
    RequestType::new("RoomDetailedInfoStream");

impl StreamHandler<(), RoomDetailedInfo> for RoomDetailedInfoStreamHandler {
    fn handle(
        &self,
        uid: u32,
        _req: (),
    ) -> BoxFuture<Result<Pin<Box<dyn Stream<Item = RoomDetailedInfo> + Send + 'static>>, Error>>
    {
        async move {
            let room = room_manager()
                .find_room_by_user_id(uid)
                .ok_or(anyhow!("user not in room"))?;
            let mut watch_recv = room.read().detailed_info_change_watch.clone_recv();
            let (send, recv) = futures_channel::mpsc::unbounded::<RoomDetailedInfo>();
            spawn(async move {
                loop {
                    {
                        let cur = watch_recv.borrow_and_update();
                        if let Some(ref info) = *cur {
                            if let Err(_) = send.unbounded_send(info.clone()) {
                                break;
                            }
                        }
                    }
                    if let Err(_) = watch_recv.changed().await {
                        break;
                    }
                }
            });
            let stream: Pin<Box<dyn Stream<Item = RoomDetailedInfo> + Send + 'static>> =
                Box::pin(recv);
            Ok(stream)
        }
        .boxed()
    }
}

pub struct AllRoomSimpleInfoStreamHandler;

pub const ALL_ROOM_SIMPLE_INFO_STREAM_TYPE: RequestType<(), Vec<RoomSimpleInfo>> =
    RequestType::new("AllRoomSimpleInfoStream");

impl StreamHandler<(), Vec<RoomSimpleInfo>> for AllRoomSimpleInfoStreamHandler {
    fn handle(
        &self,
        uid: u32,
        req: (),
    ) -> BoxFuture<Result<Pin<Box<dyn Stream<Item = Vec<RoomSimpleInfo>> + Send + 'static>>, Error>>
    {
        todo!()
    }
}
