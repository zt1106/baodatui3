use std::sync::Arc;
use rsocket_rust::prelude::{Flux, Payload, RSocket};
use rsocket_rust::stream;
use rsocket_rust::async_trait;

// per user connection
#[derive(Clone)]
pub struct ServerRSocket {
    pub client_rsocket: Arc<dyn RSocket>,
    pub user_id: u32,
}

#[async_trait]
impl RSocket for ServerRSocket {
    async fn metadata_push(&self, req: Payload) -> anyhow::Result<()> {
        Ok(())
    }

    async fn fire_and_forget(&self, req: Payload) -> anyhow::Result<()> {
        Ok(())
    }

    async fn request_response(&self, req: Payload) -> anyhow::Result<Option<Payload>> {
        Ok(None)
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
        todo!()
    }
}
