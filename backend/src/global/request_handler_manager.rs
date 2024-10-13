use crate::transport::request::handler::{RawRequestHandler, RequestHandler, RequestHandlerWrapper};
use parking_lot::RwLock;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};

pub fn request_handler_manager() -> &'static RequestHandlerManager {
    static USER_MANAGER: OnceLock<RequestHandlerManager> = OnceLock::new();
    USER_MANAGER.get_or_init(RequestHandlerManager::default)
}

/// accept handlers, convert to raw handlers
#[derive(Default)]
pub struct RequestHandlerManager {
    raw_map: Arc<RwLock<HashMap<String, Arc<dyn RawRequestHandler + Send + Sync>>>>,
}

impl RequestHandlerManager {
    pub fn add_handler<Req, Res>(&self, command: impl Into<String>, handler: impl RequestHandler<Req, Res> + 'static)
    where
        Req: Serialize + DeserializeOwned + 'static,
        Res: Serialize + DeserializeOwned + 'static,
    {
        let command = command.into();
        if self.raw_map.read().contains_key(&command) {
            panic!("Tried to add a handler for {} twice", command);
        }
        let handler_wrapper = RequestHandlerWrapper::new(handler);
        self.raw_map.write().insert(command, Arc::new(handler_wrapper));
    }
    
    pub fn raw_handler(&self, command: &String) -> Arc<dyn RawRequestHandler + Send + Sync> {
        self.raw_map.read().get(command).unwrap().clone()
    }
}