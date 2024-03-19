use std::sync::Arc;

use anyhow::Error;
use tokio::sync::Mutex;

pub trait IntoBoxExt<T> {
    fn to_box(self) -> Box<T>;
}

impl<T> IntoBoxExt<T> for T {
    fn to_box(self) -> Box<T> {
        Box::new(self)
    }
}

pub trait IntoArcTMutex<T> {
    fn to_arc_t_mutex(self) -> Arc<Mutex<T>>;
}

impl<T> IntoArcTMutex<T> for T {
    fn to_arc_t_mutex(self) -> Arc<Mutex<T>> {
        Arc::new(Mutex::new(self))
    }
}

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
