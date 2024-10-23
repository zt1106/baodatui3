use crate::main_inner;
use crate::transport::request::RequestType;
use anyhow::Error;
use futures::Stream;
use futures_util::StreamExt;
use parking_lot::Mutex;
use rsocket_rust::prelude::{Payload, RSocket, RSocketFactory};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{json, Value};
use std::future;
use std::net::TcpListener;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use tokio::spawn;
use tokio::sync::oneshot::Sender;
use tokio::task::JoinHandle;
use tokio::time::sleep;

pub struct Server {
    shutdown_main_send: Option<Sender<()>>,
    main_join_handle: Option<JoinHandle<()>>,
    listening_port: u16,
}

fn get_available_port() -> u16 {
    let socket = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = socket.local_addr().unwrap().port();
    drop(socket);
    port
}

impl Server {
    pub fn new() -> Self {
        let listening_port = get_available_port();
        let (shutdown_main_send, recv) = tokio::sync::oneshot::channel::<()>();
        let main_join_handle = spawn(async move {
            let _ = main_inner(Some(recv), Some(listening_port)).await;
        });
        Self {
            shutdown_main_send: Some(shutdown_main_send),
            main_join_handle: Some(main_join_handle),
            listening_port,
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

    pub async fn new_and_connect() -> Self {
        let mut c = Self::new();
        c.connect().await;
        c
    }

    pub async fn new_and_connect_with_server(server: Arc<Mutex<Server>>) -> Self {
        let mut c = Self::new_with_server(server);
        c.connect().await;
        c
    }

    pub fn new_with_server(server: Arc<Mutex<Server>>) -> Self {
        Self {
            server,
            r_client: None,
        }
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
                rsocket_rust_transport_websocket::WebsocketClientTransport::from(
                    format!("127.0.0.1:{}", self.server.lock().listening_port).as_str(),
                ),
            )
            .setup(
                Payload::builder()
                    .set_data_utf8(serde_json::to_string(&setup_json).unwrap().as_str())
                    .build(),
            )
            .mime_type("text/plain", "text/plain")
            .start()
            .await
            .unwrap();
        self.r_client = Some(r_client);
    }

    fn r_client(&self) -> rsocket_rust::Client {
        self.r_client.as_ref().unwrap().clone()
    }

    pub fn server(&self) -> Arc<Mutex<Server>> {
        self.server.clone()
    }

    pub fn shutdown_client(self) {}

    pub async fn request<Req, Res>(
        &self,
        req_type: RequestType<Req, Res>,
        req: &Req,
    ) -> Result<Res, Error>
    where
        Req: Serialize + DeserializeOwned,
        Res: Serialize + DeserializeOwned,
    {
        let req_v = serde_json::to_value(req)?;
        let mut req_builder = Payload::builder().set_metadata_utf8(req_type.command);
        match req_v {
            Value::Null => {}
            _ => {
                req_builder = req_builder.set_data_utf8(serde_json::to_string(&req_v)?.as_str());
            }
        }
        let res_p = self
            .r_client()
            .request_response(req_builder.build())
            .await?
            .ok_or(Error::msg("res is none"))?;
        let res_v = match res_p.data_utf8() {
            None => Ok(Value::Null),
            Some(s) => serde_json::from_str(s),
        }?;
        Ok(serde_json::from_value(res_v)?)
    }

    pub async fn request_no_args<Res>(&self, req_type: RequestType<(), Res>) -> Result<Res, Error>
    where
        Res: Serialize + DeserializeOwned,
    {
        self.request(req_type, &()).await
    }

    pub async fn stream<Req, T>(
        &self,
        req_type: RequestType<Req, T>,
        req: &Req,
    ) -> Result<Pin<Box<dyn Stream<Item = T>>>, Error>
    where
        Req: Serialize + DeserializeOwned,
        T: Serialize + DeserializeOwned + 'static,
    {
        let req_v = serde_json::to_value(req)?;
        let mut req_builder = Payload::builder().set_metadata_utf8(req_type.command);
        match req_v {
            Value::Null => {}
            _ => {
                req_builder = req_builder.set_data_utf8(serde_json::to_string(&req_v)?.as_str());
            }
        }
        let stream = self.r_client().request_stream(req_builder.build());
        let mapped = stream.filter_map(|t| {
            let r = match t {
                Ok(p) => match p.data_utf8() {
                    None => None,
                    Some(v) => serde_json::from_str::<T>(v).ok(),
                },
                Err(_) => None,
            };
            future::ready(r)
        });
        let boxed: Pin<Box<dyn Stream<Item = T>>> = Box::pin(mapped);
        Ok(boxed)
    }

    pub async fn stream_no_args<T>(
        &self,
        req_type: RequestType<(), T>,
    ) -> Result<Pin<Box<dyn Stream<Item = T>>>, Error>
    where
        T: Serialize + DeserializeOwned + 'static,
    {
        self.stream(req_type, &()).await
    }
}
