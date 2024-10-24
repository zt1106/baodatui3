pub mod data_structure;
pub mod engine;
pub mod event;
pub mod ext;
pub mod global;
pub mod model;
pub mod rsocket;
pub mod test_client;
pub mod transport;
pub mod utils;

use crate::global::handlers::room_handlers::{
    AllRoomSimpleInfoStreamHandler, ChangeGameConfigHandler, CreateRoomHandler, EnterRoomHandler,
    LeaveRoomHandler, ListRoomSimpleInfoHandler, RoomDetailedInfoStreamHandler,
    ALL_ROOM_SIMPLE_INFO_STREAM_TYPE, CHANGE_GAME_CONFIG_REQ_TYPE, CREATE_ROOM_REQ_TYPE,
    ENTER_ROOM_REQ_TYPE, LEAVE_ROOM_REQ_TYPE, LIST_ROOM_SIMPLE_INFO_REQ_TYPE,
    ROOM_DETAILED_INFO_STREAM_TYPE,
};
use crate::global::handlers::user_handlers::{
    ChangeCurUserNameHandler, GetCurUserHandler, CHANGE_CUR_USER_NAME_REQ_TYPE,
    GET_CUR_USER_REQ_TYPE,
};
use crate::global::rsocket_manager::rsocket_manager;
use crate::model::user::User;
use crate::rsocket::ServerRSocket;
use futures::executor;
use global::user_manager::user_manager;
use parking_lot::RwLock;
use rsocket_rust::prelude::*;
use rsocket_rust::Result;
use rsocket_rust_transport_websocket::WebsocketServerTransport;
use serde_json::Value;
use std::sync::Arc;
use tokio::select;
use tokio::sync::oneshot;
use utils::cur_timestamp;

const DEFAULT_SERVER_LOCAL_PORT: u16 = 8080;

pub async fn main_inner(
    stop_signal_recv: Option<oneshot::Receiver<()>>,
    port: Option<u16>,
) -> Result<()> {
    init_global_handlers();
    let server_future = RSocketFactory::receive()
        .acceptor(Box::new(|setup, client_rsocket| {
            let user_id: u32 = executor::block_on(async {
                let mut inner: Option<Arc<RwLock<User>>> = None;
                if let Some(data) = setup.data() {
                    let data_string = String::from_utf8_lossy(data);
                    let setup_r = serde_json::from_str::<Value>(&data_string);
                    if let Ok(setup) = setup_r {
                        if let Some(uuid) = setup.get("uuid") {
                            if let Some(uuid) = uuid.as_str() {
                                if let Some(user) =
                                    user_manager().find_user_by_uuid(&uuid.to_string())
                                {
                                    return user.read().id;
                                }
                            }
                        }
                    }
                }
                if inner.is_none() {
                    inner = Some(user_manager().add_default());
                }
                let inner: Arc<RwLock<User>> = inner.unwrap();
                let mut u = inner.write();
                u.login_timestamp = cur_timestamp();
                drop(u);
                let id = inner.read().id;
                id
            });
            Ok(Box::new(ServerRSocket {
                client_rsocket: Arc::from(client_rsocket),
                user_id,
            }))
        }))
        .transport(WebsocketServerTransport::from(format!(
            "127.0.0.1:{}",
            port.unwrap_or(DEFAULT_SERVER_LOCAL_PORT)
        )))
        .serve();
    if stop_signal_recv.is_none() {
        return server_future.await;
    }
    select! {
        _ = server_future => {
            Ok(())
        }
        _ = stop_signal_recv.unwrap() => {
            println!("Stop signal received, shutting down");
            Ok(())
        }
    }
}

fn init_global_handlers() {
    // users
    rsocket_manager().add_request_handler(GET_CUR_USER_REQ_TYPE, GetCurUserHandler);
    rsocket_manager().add_request_handler(CHANGE_CUR_USER_NAME_REQ_TYPE, ChangeCurUserNameHandler);

    // rooms
    rsocket_manager().add_request_handler(CREATE_ROOM_REQ_TYPE, CreateRoomHandler);
    rsocket_manager()
        .add_request_handler(LIST_ROOM_SIMPLE_INFO_REQ_TYPE, ListRoomSimpleInfoHandler);
    rsocket_manager().add_request_handler(LEAVE_ROOM_REQ_TYPE, LeaveRoomHandler);
    rsocket_manager().add_request_handler(ENTER_ROOM_REQ_TYPE, EnterRoomHandler);
    rsocket_manager().add_request_handler(CHANGE_GAME_CONFIG_REQ_TYPE, ChangeGameConfigHandler);
    rsocket_manager().add_stream_handler(
        ALL_ROOM_SIMPLE_INFO_STREAM_TYPE,
        AllRoomSimpleInfoStreamHandler,
    );
    rsocket_manager().add_stream_handler(
        ROOM_DETAILED_INFO_STREAM_TYPE,
        RoomDetailedInfoStreamHandler,
    );

    // games
}
