use anyhow::Error;
use futures_util::future::BoxFuture;
use futures_util::FutureExt;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;

pub trait RawRequestHandler {
    fn handle_raw(&self, uid: u32, val: Value) -> BoxFuture<Result<Value, Error>>;
}

pub trait RequestHandler<Req, Res>: Send + Sync
where
    Req: Serialize + DeserializeOwned,
    Res: Serialize + DeserializeOwned,
{
    fn handle(&self, uid: u32, req: Req) -> BoxFuture<Result<Res, Error>>;
}

pub struct RequestHandlerWrapper<Req, Res>
where
    Req: Serialize + DeserializeOwned,
    Res: Serialize + DeserializeOwned,
{
    inner: Box<dyn RequestHandler<Req, Res>>,
}

impl<Req, Res> RequestHandlerWrapper<Req, Res>
where
    Req: Serialize + DeserializeOwned,
    Res: Serialize + DeserializeOwned,
{
    pub fn new(handler: impl RequestHandler<Req, Res> + Send + Sync + 'static) -> Self {
        Self {
            inner: Box::new(handler),
        }
    }
}

impl<Req, Res> RawRequestHandler for RequestHandlerWrapper<Req, Res>
where
    Req: Serialize + DeserializeOwned,
    Res: Serialize + DeserializeOwned,
{
    fn handle_raw(&self, uid: u32, val: Value) -> BoxFuture<Result<Value, Error>> {
        async move {
            let req = serde_json::from_value::<Req>(val).map_err(Error::from)?;
            let resp_result = self.inner.handle(uid, req).await?;
            let resp_val = serde_json::to_value(resp_result).map_err(Error::from)?;
            Ok(resp_val)
        }.boxed()
    }
}