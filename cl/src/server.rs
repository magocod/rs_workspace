use opencl3::types::cl_int;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
pub struct SaveRequest {
    pub value: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct GetRequest {
    pub index: cl_int,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Payload {
    pub index: cl_int,
    pub value: String,
}
