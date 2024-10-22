use anyhow::Error;
use async_trait::async_trait;
use std::future::Future;
use std::pin::Pin;

pub trait IntoResult<T> {
    fn into_result(self) -> Result<T, Error>;
}

impl<T> IntoResult<T> for Option<T> {
    fn into_result(self) -> Result<T, Error> {
        match self {
            Some(t) => Ok(t),
            None => Err(Error::msg("None")),
        }
    }
}

#[async_trait]
pub trait AsyncMap<T, U, F>
where
    F: FnOnce(T) -> Pin<Box<dyn Future<Output = U> + Send>> + Send,
{
    type Output;
    async fn async_map(self, map: F) -> Self::Output;
}

#[async_trait]
impl<T, U, F> AsyncMap<T, U, F> for Option<T>
where
    T: Send,
    U: Send,
    F: 'static + FnOnce(T) -> Pin<Box<dyn Future<Output = U> + Send>> + Send,
{
    type Output = Option<U>;
    async fn async_map(self, map: F) -> Self::Output {
        match self {
            Some(t) => {
                let u = map(t).await;
                Some(u)
            }
            None => None,
        }
    }
}
