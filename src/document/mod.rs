use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Document {
    pub tree: Node,
    pub authors: HashMap<Uuid, String>,
    pub models: HashMap<Uuid, String>,
}

impl Document {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Generator {
    pub label: String,
    pub backend: String,
    pub config: Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    pub children: Vec<Node>,
    pub contents: Vec<NodeContent>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum NodeContent {
    Written(Authored<Value>),
    Comment(Authored<String>),
    Generated(Generated<Value, Value>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Authored<T> {
    pub author: Uuid,
    pub modified: DateTime<Utc>,
    pub content: T,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Generated<I, O> {
    pub generator: Uuid,
    pub start: DateTime<Utc>,
    pub finish: DateTime<Utc>,
    pub input: I,
    pub output: O,
}
