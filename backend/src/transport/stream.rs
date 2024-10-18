use std::pin::Pin;
use std::sync::Arc;
use anyhow::Error;
use futures::Stream;
use futures_channel::mpsc::UnboundedReceiver;
use futures_util::{SinkExt, StreamExt};
use futures_util::future::BoxFuture;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use tokio::spawn;

pub trait StreamHandler<Req, T>: Send + Sync
where
    Req: Serialize + DeserializeOwned,
    T: Serialize + DeserializeOwned,
{
    fn handle(&self, req: Req) -> BoxFuture<Result<Pin<Box<dyn Stream<Item=T> + Send + 'static>>, Error>>;
}

pub trait RawStreamHandler {
    fn handle(&self, req: Value) -> Result<UnboundedReceiver<Value>, Error>;
}

pub struct StreamHandlerWrapper<Req, T>
where
    Req: Serialize + DeserializeOwned + Send,
    T: Serialize + DeserializeOwned,
{
    inner: Arc<dyn StreamHandler<Req, T> + Send + Sync + 'static>,
}

impl<Req, T> StreamHandlerWrapper<Req, T>
where
    Req: Serialize + DeserializeOwned + Send,
    T: Serialize + DeserializeOwned,
{
    pub(crate) fn new(inner: impl StreamHandler<Req, T> + Send + Sync + 'static) -> Self {
        Self {
            inner: Arc::new(inner),
        }
    }
}

impl<Req, T> RawStreamHandler for StreamHandlerWrapper<Req, T>
where
    Req: Serialize + DeserializeOwned + 'static + Send,
    T: Serialize + DeserializeOwned + Send + 'static,
{
    fn handle(&self, req: Value) -> Result<UnboundedReceiver<Value>, Error> {
        let (mut send, recv) = futures_channel::mpsc::unbounded::<Value>();
        let req = serde_json::from_value::<Req>(req).map_err(|e| Error::from(e))?;
        let inner = self.inner.clone();
        spawn(async move {
            let stream_f = inner.handle(req);
            let mut stream = stream_f.await?;
            loop {
                let next = stream.next().await;
                match next {
                    None => break,
                    Some(next) => {
                        let next_v = serde_json::to_value(next)?;
                        match send.send(next_v).await {
                            Ok(_) => {}
                            Err(_) => {
                                break;
                            }
                        }
                    }
                }
            }
            Ok::<(), Error>(())
        });
        Ok(recv)
    }
}
