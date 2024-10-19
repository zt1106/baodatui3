use parking_lot::RwLock;
use std::sync::atomic::AtomicU32;
use std::{collections::HashMap, sync::Arc};

pub trait WithId {
    fn set_id(&mut self, id: u32);
    fn id(&self) -> u32;
}

impl<T: WithId> WithId for Box<T> {
    fn set_id(&mut self, id: u32) {
        self.as_mut().set_id(id);
    }

    fn id(&self) -> u32 {
        self.as_ref().id()
    }
}

/// simple map implementation used as in-memory database
pub struct GlobalMap<T> {
    cur_id: AtomicU32,
    inner_map: RwLock<HashMap<u32, Arc<RwLock<T>>>>,
}

impl<T> Default for GlobalMap<T> {
    fn default() -> Self {
        Self {
            cur_id: Default::default(),
            inner_map: Default::default(),
        }
    }
}

impl<T: WithId + Default> GlobalMap<T> {
    pub fn add_default(&self) -> Arc<RwLock<T>> {
        self.add(T::default())
    }
}

impl<T: WithId> GlobalMap<T> {
    pub fn add(&self, mut t: T) -> Arc<RwLock<T>> {
        let mut map = self.inner_map.write();
        let cur_id = self
            .cur_id
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        t.set_id(cur_id);
        let arc = Arc::new(RwLock::new(t));
        let arc_cloned = arc.clone();
        map.insert(cur_id, arc);
        arc_cloned
    }

    pub fn remove_id(&self, id: u32) {
        let mut map = self.inner_map.write();
        map.remove(&id);
    }

    pub fn remove(&self, t: Arc<RwLock<T>>) {
        let id = t.read().id();
        self.remove_id(id);
    }

    pub fn get(&self, id: u32) -> Option<Arc<RwLock<T>>> {
        let map = self.inner_map.read();
        map.get(&id).and_then(|t| Some(t.clone()))
    }

    pub fn contains_id(&self, id: u32) -> bool {
        self.inner_map.read().contains_key(&id)
    }

    pub fn find(&self, f: impl Fn(&T) -> bool) -> Option<Arc<RwLock<T>>> {
        let map = self.inner_map.read();
        for (_, v) in map.iter() {
            let t = v.read();
            if f(&*t) {
                return Some(v.clone());
            }
        }
        None
    }
}
