use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct PayloadRequest {
    pub opt: String,
    pub value: String
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Payload {
    pub index: u64,
    pub value: String,
}
