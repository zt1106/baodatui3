use crate::ext::IntoArcTMutex;
use std::{collections::HashMap, sync::Arc};

use std::sync::Mutex as StdMutex;
use tokio::sync::Mutex;

pub trait WithId {
    fn set_id(&mut self, id: u32);
    fn id(&self) -> u32;
}

/// both map itself and values are wrapped in Arc + Mutex, maybe a bit overkill
#[derive(Default)]
pub struct ArcMap<T: WithId> {
    cur_id: Arc<StdMutex<u32>>,
    inner_map: Arc<StdMutex<HashMap<u32, Arc<Mutex<T>>>>>,
}

impl<T: WithId> ArcMap<T> {
    pub fn add(&self, mut t: T) -> Arc<Mutex<T>> {
        let mut map = self.inner_map.lock().unwrap();
        let mut cur_id = self.cur_id.lock().unwrap();
        t.set_id(*cur_id);
        let arc = t.to_arc_t_mutex();
        let arc_cloned = arc.clone();
        map.insert(*cur_id, arc);
        *cur_id += 1;
        arc_cloned
    }

    pub fn remove_id(&self, id: u32) {
        let mut map = self.inner_map.lock().unwrap();
        map.remove(&id);
    }

    pub async fn remove(&self, t: Arc<Mutex<T>>) {
        let id = t.lock().await.id();
        self.remove_id(id);
    }

    pub fn get(&self, id: u32) -> Option<Arc<Mutex<T>>> {
        let map = self.inner_map.lock().unwrap();
        map.get(&id).and_then(|t| Some(t.clone()))
    }

    pub fn is_valid_id(&self, id: u32) -> bool {
        self.inner_map.lock().unwrap().contains_key(&id)
    }

    pub async fn find(&self, f: impl Fn(&T) -> bool) -> Option<Arc<Mutex<T>>> {
        let map = self.inner_map.lock().unwrap();
        for (_, v) in map.iter() {
            let t = v.lock().await;
            if f(&*t) {
                return Some(v.clone());
            }
        }
        None
    }
}
