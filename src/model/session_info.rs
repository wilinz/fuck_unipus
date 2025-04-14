use serde::{Deserialize, Serialize};

#[derive(Clone ,Serialize, Deserialize, Debug)]
#[allow(dead_code)]
pub struct SessionInfo {
    pub name: String,
    pub token: String,
}