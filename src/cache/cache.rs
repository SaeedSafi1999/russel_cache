use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct Cache {
    store: Arc<Mutex<HashMap<String, HashMap<String, Vec<u8>>>>>,
    default_port: u16,
    //max_ram_limit:u16
}


impl Cache {
    pub fn new() -> Self {
        Cache {
            store: Arc::new(Mutex::new(HashMap::new())),
            default_port: 5022, // Default port configuration
            //max_ram_limit:max_ram_limit
        }
    }

    pub fn set(&self, cluster: String, key: String, value: Vec<u8>) {
        let mut store = self.store.lock().unwrap();
        let cluster_store = store.entry(cluster).or_insert_with(HashMap::new);
        cluster_store.insert(key, value);
    }

    pub fn set_cluster(&self, cluster: String) {
        let mut store = self.store.lock().unwrap();
        store.entry(cluster).or_insert_with(HashMap::new);
    }

    pub fn get(&self, cluster: &str, key: &str) -> Option<Vec<u8>> {
        let store = self.store.lock().unwrap();
        store.get(cluster).and_then(|cluster_store| cluster_store.get(key).cloned())
    }

    pub fn get_keys_of_cluster(&self, cluster: &str) -> Option<Vec<String>> {
        let store = self.store.lock().unwrap();
        store.get(cluster).map(|cluster_store| cluster_store.keys().cloned().collect())
    }
    
    pub fn delete(&self, cluster: &str, key: &str) {
        let mut store = self.store.lock().unwrap();
        if let Some(cluster_store) = store.get_mut(cluster) {
            cluster_store.remove(key);
        }
    }

    pub fn clear_cluster(&self, cluster: &str) {
        let mut store = self.store.lock().unwrap();
        store.remove(cluster);
    }

    pub fn clear_all(&self) {
        let mut store = self.store.lock().unwrap();
        store.clear();
    }

    pub fn get_all_clusters(&self) -> Vec<String> {
        let store = self.store.lock().unwrap();
        store.keys().cloned().collect()
    }

    pub fn configure_default_port(&mut self, port: u16) {
        self.default_port = port;
    }

    pub fn get_default_port(&self) -> u16 {
        self.default_port
    }
}
