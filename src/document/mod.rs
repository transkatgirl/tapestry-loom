use std::{collections::HashMap, sync::Arc};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use tokio::sync::{OwnedRwLockReadGuard, OwnedRwLockWriteGuard, RwLock};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Document {
    pub tree: Wrapper<Node>,
    pub labels: Wrapper<HashMap<Uuid, String>>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Node {
    pub children: Vec<Wrapper<Node>>,
    pub contents: Vec<NodeContent>,
}

#[derive(Debug, Default, Clone)]
pub struct Wrapper<T>(Arc<RwLock<T>>);

impl<T> Wrapper<T> {
    pub fn new(value: T) -> Self {
        Wrapper(Arc::new(RwLock::new(value)))
    }

    pub async fn read(&self) -> OwnedRwLockReadGuard<T> {
        self.0.clone().read_owned().await
    }

    pub async fn write(&self) -> OwnedRwLockWriteGuard<T> {
        self.0.clone().write_owned().await
    }
}

impl<T: Serialize> Serialize for Wrapper<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        T::serialize(&*self.0.blocking_read(), serializer)
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Wrapper<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Wrapper::new(T::deserialize(deserializer)?))
    }
}

impl Node {
    pub fn shallow_clone(&self) -> Self {
        Self {
            children: Vec::new(),
            contents: self.contents.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum NodeContent {
    Written(Authored<Value>),
    Comment(Authored<String>),
    Generated(Generated<Value, Value>),
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Authored<T> {
    pub author: Uuid,
    pub modified: DateTime<Utc>,
    pub content: T,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Generated<I, O> {
    pub generator: Uuid,
    pub start: DateTime<Utc>,
    pub finish: DateTime<Utc>,
    pub input: I,
    pub output: O,
}
