use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Content {
    Written(Authored<Value>),
    Comment(Authored<String>),
    Generated(Generated<Value, Value>),
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SharedMetadata {
    pub labels: HashMap<Uuid, String>,
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
