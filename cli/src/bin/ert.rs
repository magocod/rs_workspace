use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataStoreError {
    #[error("data store disconnected")]
    Disconnect(#[from] io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("unknown data store error")]
    Unknown,
}

fn call_error() -> Result<(), DataStoreError> {
    Err(DataStoreError::Unknown)
}

fn handle_error() -> Result<(), DataStoreError> {
    call_error()?;
    Ok(())
}

fn main() {
    let r = DataStoreError::Redaction("prop".to_string());
    println!("{r}");

    let r = call_error();
    println!("{r:?}");

    let r = handle_error();
    println!("{r:?}");
}
