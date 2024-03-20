pub mod container;
pub mod ext;
pub mod model;
pub mod rsocket;
mod test;
pub mod utils;
pub mod serialize;
pub mod handler;
pub mod event;
pub mod db;
pub mod pattern;
pub mod mode;
use crate::container::user_manager::user_manager;
use crate::model::user::User;
use crate::rsocket::ServerRSocket;
use futures::executor;
use rsocket_rust::prelude::*;
use rsocket_rust::Result;
use rsocket_rust_transport_websocket::WebsocketServerTransport;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::Mutex;
use utils::cur_timestamp;

const ADDR: &str = "127.0.0.1:8080";

#[tokio::main]
async fn main() -> Result<()> {
    RSocketFactory::receive()
        .acceptor(Box::new(|setup, client_rsocket| {
            let user: Arc<Mutex<User>> = executor::block_on(async {
                let mut inner: Option<Arc<Mutex<User>>> = None;
                if let Some(data) = setup.data() {
                    let data_string = String::from_utf8_lossy(data);
                    let setup_r = serde_json::from_str::<Value>(&data_string);
                    if let Ok(setup) = setup_r {
                        if let Some(uuid) = setup.get("uuid") {
                            if let Some(uuid) = uuid.as_str() {
                                if let Some(user) = user_manager().find_user_by_uuid(uuid).await {
                                    return user;
                                }
                            }
                        }
                    }
                }
                if inner.is_none() {
                    inner = Some(user_manager().add_user().await);
                }
                let inner: Arc<Mutex<User>> = inner.unwrap();
                let mut u = inner.lock().await;
                u.login_timestamp = cur_timestamp();
                drop(u);
                inner
            });
            Ok(Box::new(ServerRSocket {
                client_rsocket,
                user,
            }))
        }))
        .transport(WebsocketServerTransport::from(ADDR))
        .serve()
        .await
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
