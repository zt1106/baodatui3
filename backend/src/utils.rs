use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::watch::{Receiver, Sender};

pub fn cur_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// a wrapper of tokio watch channel
/// only the initial value is None
pub struct WatcherWrapper<T> {
    send: Sender<Option<T>>,
    recv: Receiver<Option<T>>,
}

impl<T> Default for WatcherWrapper<T> {
    fn default() -> Self {
        let (send, recv) = tokio::sync::watch::channel::<Option<T>>(None);
        Self { send, recv }
    }
}

impl<T> WatcherWrapper<T> {
    pub fn send(&self, value: T) {
        self.send.send(Some(value)).unwrap()
    }

    pub fn clone_recv(&self) -> Receiver<Option<T>> {
        self.recv.clone()
    }
}
