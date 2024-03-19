use std::sync::{Arc};

use async_trait::async_trait;
use rsocket_rust::prelude::StreamExt;
use rsocket_rust::prelude::{Flux, Payload, RSocket};
use rsocket_rust::runtime;
use rsocket_rust::stream;
use tokio::sync::{mpsc, Mutex};

use crate::ext::IntoResult;
use crate::handler::handler_map;
use crate::model::user::User;

pub struct ServerRSocket {
    pub client_rsocket: Box<dyn RSocket>,
    pub user: Arc<Mutex<User>>,
}

#[async_trait]
impl RSocket for ServerRSocket {
    async fn metadata_push(&self, req: Payload) -> anyhow::Result<()> {
        Ok(())
    }

    async fn fire_and_forget(&self, req: Payload) -> anyhow::Result<()> {
        Ok(())
    }

    /// TODO no need for error response
    async fn request_response(&self, req: Payload) -> anyhow::Result<Option<Payload>> {
        let raw_request = crate::serialize::payload_to_raw_request(&req)?;
        let handler = handler_map().get(&raw_request.command).into_result()?;
        let user = self.user.lock().await;
        let raw_response = handler.raw_handle(raw_request.data, user.id).await;
        Ok(Some(crate::serialize::raw_response_to_payload(raw_response)?))
    }

    fn request_stream(&self, req: Payload) -> Flux<anyhow::Result<Payload>> {
        Box::pin(stream! {
            yield Ok(req);
        })
    }

    fn request_channel(
        &self,
        mut reqs: Flux<anyhow::Result<Payload>>,
    ) -> Flux<anyhow::Result<Payload>> {
        let (sender, mut receiver) = mpsc::unbounded_channel();
        runtime::spawn(async move {
            while let Some(it) = reqs.next().await {
                sender.send(it).unwrap();
            }
        });
        Box::pin(stream! {
            while let Some(it) = receiver.recv().await {
                yield it;
            }
        })
    }
}
