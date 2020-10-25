use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use serde::{Serialize, Deserialize};

pub type Songs = HashMap<String, u32>;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Id {
    pub title: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Song {
    pub title: String,
    pub duration: u32,
}

#[derive(Clone)]
pub struct Store {
  pub list: Arc<RwLock<Songs>>
}

impl Store {
    pub fn new() -> Self {
        Store {
            list: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}