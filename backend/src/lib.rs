pub mod ext;
pub mod model;
pub mod rsocket;
pub mod utils;
pub mod global;
pub mod data_structure;
pub mod transport;
pub mod event;
pub mod engine;

use global::user_manager::user_manager;
use crate::model::user::User;
use crate::rsocket::ServerRSocket;
use futures::executor;
use rsocket_rust::prelude::*;
use rsocket_rust::Result;
use rsocket_rust_transport_websocket::WebsocketServerTransport;
use serde_json::Value;
use std::sync::Arc;
use tokio::select;
use tokio::sync::{oneshot, Mutex, RwLock};
use utils::cur_timestamp;

const ADDR: &str = "127.0.0.1:8080";

pub async fn main_inner(stop_signal_recv: Option<oneshot::Receiver<()>>) -> Result<()> {
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
                                if let Some(user) = user_manager().find_user_by_uuid(uuid).await {
                                    return user.read().await.id;
                                }
                            }
                        }
                    }
                }
                if inner.is_none() {
                    inner = Some(user_manager().add_default());
                }
                let inner: Arc<RwLock<User>> = inner.unwrap();
                let mut u = inner.write().await;
                u.login_timestamp = cur_timestamp();
                drop(u);
                let id = inner.read().await.id;
                id
            });
            Ok(Box::new(ServerRSocket {
                client_rsocket: Arc::from(client_rsocket),
                user_id,
            }))
        }))
        .transport(WebsocketServerTransport::from(ADDR))
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

#[tokio::test]
async fn test_connect() -> Result<()> {
    let client = RSocketFactory::connect()
        .transport(
            rsocket_rust_transport_websocket::WebsocketClientTransport::from("127.0.0.1:8080"),
        )
        .setup(Payload::from("READY!"))
        .mime_type("text/plain", "text/plain")
        .start()
        .await?;

    let request_payload = Payload::builder()
        .set_data_utf8("Hello World!")
        .set_metadata_utf8("Rust")
        .build();

    let res = client.request_response(request_payload).await?;
    println!("got response: {:?}", res);
    Ok(())
}
