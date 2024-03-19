use std::collections::HashMap;

use async_trait::async_trait;
use serde_json::Value;
use std::sync::OnceLock;

use crate::serialize::RawResponse;

pub mod room_handler;
pub mod user_handler;

static HANDLER_MAP: OnceLock<HashMap<String, Box<dyn RawHandler + Sync + Send>>> = OnceLock::new();

pub fn handler_map() -> &'static HashMap<String, Box<dyn RawHandler + Sync + Send>> {
    HANDLER_MAP.get_or_init(|| {
        let mut map: HashMap<String, Box<dyn RawHandler + Sync + Send>> = HashMap::new();
        map.insert("changeUserName".to_string(), Box::new(RawHandlerImpl::new(Box::new(user_handler::ChangeUserNameHandler))));
        map
    })
}

/// typed response
pub enum Response<T> {
    Data(T),
    Error(String),
}

#[async_trait]
pub trait RawHandler {
    async fn raw_handle(&self, raw_request_data: Value, uid: u32) -> RawResponse;
}

#[async_trait]
pub trait Handler<RQ, RP> {
    async fn handle(&self, request_data: RQ, uid: u32) -> Response<RP>;
}

pub struct RawHandlerImpl<RQ: serde::de::DeserializeOwned, RP: serde::Serialize> {
    handler: Box<dyn Handler<RQ, RP> + Sync + Send>,
}

impl <RQ: serde::de::DeserializeOwned, RP: serde::Serialize> RawHandlerImpl<RQ, RP> {
    pub fn new(handler: Box<dyn Handler<RQ, RP> + Sync + Send>) -> Self {
        Self {
            handler,
        }
    }
}

#[async_trait]
impl<RQ: serde::de::DeserializeOwned, RP: serde::Serialize> RawHandler for RawHandlerImpl<RQ, RP> {
    async fn raw_handle(& self, raw_request_data: Value, uid: u32) -> RawResponse {
        let request = serde_json::from_value(raw_request_data).unwrap();
        let response = self.handler.handle(request, uid).await;
        match response {
            Response::Data(data) => {
                let data = serde_json::to_value(data).unwrap();
                RawResponse {
                    data: Some(data),
                    success: true,
                    error: None,
                }
            }
            Response::Error(error) => {
                RawResponse {
                    data: None,
                    success: false,
                    error: Some(error),
                }
            }
        }
    }
}
