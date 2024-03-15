use std::{collections::HashMap, sync::{Arc, Mutex}};

pub trait Storage<T: Clone> {
    fn get(&self, id: &str) -> Option<T>;
    fn add(&self, id: &str, value: T);
    fn remove(&self, id: &str);
}

#[derive(Clone)]
pub struct InMemoryStorage<T> {
    data: Arc<Mutex<HashMap<String, T>>>,
}

impl <T> Default for InMemoryStorage<T> {
    fn default() -> Self {
        InMemoryStorage {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl<T: Clone> Storage<T> for InMemoryStorage<T> {
    fn get(&self, id: &str) -> Option<T> {
        let data = self.data.lock().unwrap();
        data.get(id).cloned()
    }

    fn add(&self, id: &str, value: T) {
        let mut data = self.data.lock().unwrap();
        data.insert(id.to_string(), value);
    }

    fn remove(&self, id: &str) {
        let mut data = self.data.lock().unwrap();
        data.remove(id);
    }
}
