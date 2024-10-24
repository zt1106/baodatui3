use std::ops::{Deref, DerefMut};
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

pub struct RefWrapper<'a, T, S>
where
    S: From<&'a mut T>,
{
    inner: &'a mut T,
    _marker: std::marker::PhantomData<S>,
}

impl<'a, T, S> RefWrapper<'a, T, S>
where
    S: From<&'a mut T>,
{
    pub fn new(inner: &'a mut T) -> Self {
        Self {
            inner,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn as_s(&'a mut self) -> S {
        let a = self.inner.deref_mut();
        S::from(a)
    }
}

impl<'a, T, S> Deref for RefWrapper<'a, T, S>
where
    S: From<&'a mut T>,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

impl<'a, T, S> DerefMut for RefWrapper<'a, T, S>
where
    S: From<&'a mut T>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner
    }
}

impl<'a, T, S> Drop for RefWrapper<'a, T, S>
where
    S: From<&'a mut T>,
{
    fn drop(&mut self) {
        let s = self.as_s();
    }
}

// pub fn test(s: &mut String) -> impl DerefMut<Target=String> + '_ {
//     let wrapped = RefWrapper::new(s);
//     wrapped
// }
//
// fn call_test() {
//     let mut s: String = String::new();
//     let mut w = test(&mut s);
//     w.push_str("world");
// }
