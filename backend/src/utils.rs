use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::spawn;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::watch::Receiver;
use tokio::time::sleep;

enum DebouncePolicy {
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

impl<T: Send + Sync + 'static> Default for WatcherWrapper<T> {
    fn default() -> Self {
        let (send, recv) = tokio::sync::watch::channel::<Option<T>>(None);
        let (buffered_send, mut buffered_recv) = tokio::sync::mpsc::unbounded_channel::<T>();
        let policy = DebouncePolicy::default();
        match policy {
            DebouncePolicy::OnlySendLast(timeout) => {
                spawn(async move {
                    loop {
                        let next = buffered_recv.recv().await;
                        if next.is_none() {
                            dbg!("next is none");
                            break;
                        } else {
                            dbg!("next is some");
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
                            dbg!("watch send is error");
                            break;
                        } else {
                            dbg!("watch send is success");
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

impl<T> WatcherWrapper<T> {
    pub fn send(&self, value: T) {
        self.buffered_send.send(value).unwrap();
    }

    pub fn clone_recv(&self) -> Receiver<Option<T>> {
        self.recv.clone()
    }
}
