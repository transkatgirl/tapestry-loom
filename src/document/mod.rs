use std::{fmt::Display, sync::Arc};

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use tokio::sync::{OwnedRwLockReadGuard, OwnedRwLockWriteGuard, RwLock};

pub mod content;

use content::{Content, SharedMetadata};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Document<I, F, O>
where
    I: Send + Sync + Clone + Display,
    F: Send + Sync + Clone,
    O: Send + Sync + Clone + Display,
{
    pub tree: Wrapper<Node<I, F, O>>,
    pub meta: Wrapper<SharedMetadata>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Node<I, F, O>
where
    I: Send + Sync + Clone + Display,
    F: Send + Sync + Clone,
    O: Send + Sync + Clone + Display,
{
    pub children: Vec<Wrapper<Node<I, F, O>>>,
    pub contents: Vec<Content<I, F, O>>,
}

impl<I, F, O> Node<I, F, O>
where
    I: Send + Sync + Clone + Display,
    F: Send + Sync + Clone,
    O: Send + Sync + Clone + Display,
{
    pub fn shallow_clone(&self) -> Self {
        Self {
            children: Vec::new(),
            contents: self.contents.clone(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Wrapper<T: Send + Sync>(Arc<RwLock<T>>);

impl<T: Send + Sync> Wrapper<T> {
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

impl<T: Send + Sync + Serialize> Serialize for Wrapper<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        T::serialize(&*self.0.blocking_read(), serializer)
    }
}

impl<'de, T: Send + Sync + Deserialize<'de>> Deserialize<'de> for Wrapper<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Wrapper::new(T::deserialize(deserializer)?))
    }
}
