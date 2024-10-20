use crate::transport::request::{
    RawRequestHandler, RequestHandler, RequestHandlerWrapper, RequestType,
};
use crate::transport::stream::{RawStreamHandler, StreamHandler, StreamHandlerWrapper};
use parking_lot::RwLock;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};

pub fn rsocket_manager() -> &'static RSocketManager {
    static USER_MANAGER: OnceLock<RSocketManager> = OnceLock::new();
    USER_MANAGER.get_or_init(RSocketManager::default)
}

/// accept handlers, convert to raw handlers
#[derive(Default)]
pub struct RSocketManager {
    raw_req_handler_map: Arc<RwLock<HashMap<String, Arc<dyn RawRequestHandler + Send + Sync>>>>,
    raw_stream_handler_map: Arc<RwLock<HashMap<String, Arc<dyn RawStreamHandler + Send + Sync>>>>,
}

impl RSocketManager {
    pub fn add_request_handler<Req, Res>(
        &self,
        req_type: RequestType<Req, Res>,
        handler: impl RequestHandler<Req, Res> + 'static,
    ) where
        Req: Serialize + DeserializeOwned + 'static,
        Res: Serialize + DeserializeOwned + 'static,
    {
        let command = req_type.command.to_string();
        if self.raw_req_handler_map.read().contains_key(&command) {
            panic!("Tried to add a handler for {} twice", command);
        }
        let handler_wrapper = RequestHandlerWrapper::new(handler);
        self.raw_req_handler_map
            .write()
            .insert(command, Arc::new(handler_wrapper));
    }

    pub fn raw_handler(&self, command: &str) -> Option<Arc<dyn RawRequestHandler + Send + Sync>> {
        self.raw_req_handler_map.read().get(command).cloned()
    }

    pub fn add_stream_handler<Req, T>(
        &self,
        command: impl Into<String>,
        handler: impl StreamHandler<Req, T> + 'static,
    ) where
        Req: Serialize + DeserializeOwned + 'static + Send,
        T: Serialize + DeserializeOwned + Send + 'static,
    {
        let command = command.into();
        if self.raw_stream_handler_map.read().contains_key(&command) {
            panic!("Tried to add a handler for {} twice", command);
        }
        let wrapper = StreamHandlerWrapper::new(handler);
        self.raw_stream_handler_map
            .write()
            .insert(command, Arc::new(wrapper));
    }

    pub fn raw_stream_handler(&self, command: &str) -> Arc<dyn RawStreamHandler + Send + Sync> {
        self.raw_stream_handler_map
            .read()
            .get(command)
            .unwrap()
            .clone()
    }
}
