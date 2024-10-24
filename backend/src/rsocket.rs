use crate::ext::IntoResult;
use crate::global::rsocket_manager::rsocket_manager;
use async_trait::async_trait;
use futures_util::StreamExt;
use rsocket_rust::prelude::{Flux, Payload, RSocket};
use rsocket_rust::stream;
use serde_json::Value;
use std::sync::Arc;

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
        let req_v = match req.data_utf8() {
            None => Ok(Value::Null),
            Some(s) => serde_json::from_str(s),
        }?;
        let command = req.metadata_utf8().into_result()?;
        let handler = rsocket_manager()
            .raw_handler(command)
            .ok_or(anyhow::anyhow!("no handler registerd for {}", command))?;
        let resp_v = handler.handle_raw(self.user_id, req_v).await?;
        let resp_s = serde_json::to_string(&resp_v)?;
        let payload = Payload::builder().set_data_utf8(resp_s.as_str()).build();
        Ok(Some(payload))
    }

    fn request_stream(&self, req: Payload) -> Flux<anyhow::Result<Payload>> {
        let req_v = match req.data_utf8() {
            None => Ok(Value::Null),
            Some(s) => serde_json::to_value(s),
        };
        if let Err(err) = req_v {
            return Box::pin(stream! {
               yield Err(err.into());
            });
        }
        let command = req.metadata_utf8().into_result();
        if let Err(err) = command {
            return Box::pin(stream! {
               yield Err(err.into());
            });
        }
        let command = command.unwrap();
        let recv_result = rsocket_manager()
            .raw_stream_handler(command)
            .handle(self.user_id, req_v.unwrap());
        if let Err(err) = recv_result {
            return Box::pin(stream! {
               yield Err(err.into());
            });
        }
        let recv = recv_result.unwrap();
        Box::pin(recv.map(|v| {
            let s = serde_json::to_string(&v)?;
            Ok(Payload::builder().set_data_utf8(s.as_str()).build())
        }))
    }

    fn request_channel(
        &self,
        reqs: Flux<anyhow::Result<Payload>>,
    ) -> Flux<anyhow::Result<Payload>> {
        todo!()
    }
}
