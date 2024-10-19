use std::sync::Arc;
use std::time::Duration;
use anyhow::Error;
use parking_lot::Mutex;
use rsocket_rust::prelude::{Payload, RSocket, RSocketFactory};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{json, Value};
use crate::{main_inner, SERVER_LOCAL_PORT};
use tokio::spawn;
use tokio::sync::oneshot::Sender;
use tokio::task::JoinHandle;
use tokio::time::sleep;
use uuid::Uuid;
use crate::model::user::User;

pub struct Server {
    shutdown_main_send: Option<Sender<()>>,
    main_join_handle: Option<JoinHandle<()>>,
}

impl Server {
    pub fn new() -> Self {
        let (shutdown_main_send, recv) = tokio::sync::oneshot::channel::<()>();
        let main_join_handle = spawn(async {
            let _ = main_inner(Some(recv)).await;
        });
        Self {
            shutdown_main_send: Some(shutdown_main_send),
            main_join_handle: Some(main_join_handle),
        }
    }


    /// can only call once
    pub fn shutdown_send(&mut self) -> Sender<()> {
        self.shutdown_main_send.take().unwrap()
    }

    /// can only call once
    pub fn join_handle(&mut self) -> JoinHandle<()> {
        self.main_join_handle.take().unwrap()
    }
}

pub struct Client {
    server: Arc<Mutex<Server>>,
    r_client: Option<rsocket_rust::Client>,
}

impl Client {
    pub fn new() -> Self {
        Self {
            server: Arc::new(Mutex::new(Server::new())),
            r_client: None,
        }
    }

    pub fn new_with_server(server: Arc<Mutex<Server>>) -> Self {
        Self { server, r_client: None }
    }

    pub fn shutdown_server(&self) {
        self.server.lock().shutdown_send().send(()).unwrap()
    }

    pub async fn wait_server_shutdown(&self) {
        self.server.lock().join_handle().await.unwrap()
    }

    pub async fn shutdown_and_wait_server_exit(&self) {
        self.shutdown_server();
        self.wait_server_shutdown().await;
    }

    pub async fn connect(&mut self) {
        self.connect_with_uuid("").await
    }

    pub async fn connect_with_uuid(&mut self, uuid: &str) {
        // TODO how to remove this??
        sleep(Duration::from_millis(50)).await;
        let setup_json = json!({
            "uuid": uuid,
        });
        let r_client = RSocketFactory::connect()
            .transport(
                rsocket_rust_transport_websocket::WebsocketClientTransport::from(format!("127.0.0.1:{}", SERVER_LOCAL_PORT).as_str()),
            )
            .setup(Payload::builder().set_data_utf8(serde_json::to_string(&setup_json).unwrap().as_str()).build())
            .mime_type("text/plain", "text/plain")
            .start()
            .await.unwrap();
        self.r_client = Some(r_client);
    }

    fn r_client(&self) -> rsocket_rust::Client {
        self.r_client.as_ref().unwrap().clone()
    }

    pub fn server(&self) -> Arc<Mutex<Server>> {
        self.server.clone()
    }

    pub fn shutdown_client(self) {}

    pub async fn generic_request<Req, Res>(&self, command: &str, req: &Req) -> Result<Res, Error>
    where
        Req: Serialize + DeserializeOwned,
        Res: Serialize + DeserializeOwned,
    {
        let req_v = serde_json::to_value(req)?;
        let mut req_builder = Payload::builder()
            .set_metadata_utf8(command);
        match req_v {
            Value::Null => {}
            _ => {
                req_builder = req_builder.set_data_utf8(serde_json::to_string(&req_v)?.as_str());
            }
        }
        let res_p = self.r_client().request_response(req_builder.build()).await?.ok_or(Error::msg("res is none"))?;
        let res_v = match res_p.data_utf8() {
            None => Ok(Value::Null),
            Some(s) => {
                serde_json::from_str(s)
            }
        }?;
        Ok(serde_json::from_value(res_v)?)
    }

    pub async fn cur_user(&self) -> User {
        self.generic_request("GetCurUser", &()).await.unwrap()
    }
}