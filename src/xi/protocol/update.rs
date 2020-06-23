use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::xi::protocol::Operation;
use crate::xi::ViewId;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct UpdateNotification {
    pub update: Update,
    pub view_id: ViewId,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Update {
    #[serde(skip)]
    pub rev: Option<u64>,
    #[serde(rename = "ops")]
    pub operations: Vec<Operation>,
    pub annotations: Vec<Annotation>,
    pub pristine: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Annotation {
    #[serde(rename = "type")]
    pub ty: String,
    pub ranges: Vec<[u64; 4]>,
    pub payloads: Value,
    pub n: u64,
}
