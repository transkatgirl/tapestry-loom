use std::{collections::HashMap, fmt::Display};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Content<I: Display, F, O: Display> {
    Written(Authored<I>),
    Comment(Authored<String>),
    Generated(Generated<F, O>),
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SharedMetadata {
    pub authors: HashMap<Uuid, String>,
    pub generators: HashMap<Uuid, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Authored<T> {
    pub author: Uuid,
    pub modified: DateTime<Utc>,
    pub content: T,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Generated<T, O> {
    pub generator: Uuid,
    pub start: DateTime<Utc>,
    pub finish: DateTime<Utc>,
    pub config: T,
    pub output: O,
}
