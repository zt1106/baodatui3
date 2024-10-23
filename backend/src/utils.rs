use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::spawn;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::watch::Receiver;
use tokio::time::sleep;

pub enum DebouncePolicy {
    /// when an item is available, wait some time for more items, and only keep the last one
    OnlySendLast(u64),
    // TODO add more
    // None
    // OnlySendFirst
}

impl Default for DebouncePolicy {
    fn default() -> Self {
        DebouncePolicy::OnlySendLast(5u64)
    }
}

pub fn cur_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// a wrapper of tokio watch channel
/// only the initial value is None
pub struct WatcherWrapper<T> {
    recv: Receiver<Option<T>>,
    buffered_send: UnboundedSender<T>,
    policy: DebouncePolicy,
}

impl<T: Send + Sync + 'static> WatcherWrapper<T> {
    pub fn new(policy: DebouncePolicy) -> Self {
        let (send, recv) = tokio::sync::watch::channel::<Option<T>>(None);
        let (buffered_send, mut buffered_recv) = tokio::sync::mpsc::unbounded_channel::<T>();
        match policy {
            DebouncePolicy::OnlySendLast(timeout) => {
                spawn(async move {
                    loop {
                        let next = buffered_recv.recv().await;
                        if next.is_none() {
                            break;
                        }
                        let mut next = next.unwrap();
                        sleep(Duration::from_millis(timeout)).await;
                        loop {
                            if let Ok(n) = buffered_recv.try_recv() {
                                next = n;
                            } else {
                                break;
                            }
                        }
                        if let Err(_) = send.send(Some(next)) {
                            break;
                        }
                    }
                });
            }
        }
        Self {
            recv,
            buffered_send,
            policy,
        }
    }
}

impl<T: Send + Sync + 'static> Default for WatcherWrapper<T> {
    fn default() -> Self {
        Self::new(DebouncePolicy::default())
    }
}

impl<T> WatcherWrapper<T> {
    pub fn send(&self, value: T) {
        self.buffered_send.send(value).unwrap();
    }

    pub fn clone_recv(&self) -> Receiver<Option<T>> {
        self.recv.clone()
    }
}
