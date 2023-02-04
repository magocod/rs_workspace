use std::fs::read_to_string;

#[derive(Debug)]
enum MyError {
    EnvironmentVariableNotFound,
    IOError(std::io::Error),
    Other,
}

impl From<std::env::VarError> for MyError {
    fn from(_: std::env::VarError) -> Self {
        Self::EnvironmentVariableNotFound
    }
}

impl From<std::io::Error> for MyError {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value)
    }
}

impl From<OtherError> for MyError {
    fn from(_: OtherError) -> Self {
        Self::Other
    }
}

impl std::error::Error for MyError {}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyError::EnvironmentVariableNotFound => write!(f, "Environment variable not found"),
            // MyError::IOError(err) => write!(f, "IO Error: {}", err.to_string()),
            MyError::IOError(err) => write!(f, "IO Error: {err}"),
            MyError::Other => write!(f, "other error"),
        }
    }
}

#[derive(Debug)]
enum OtherError {
    Unknown,
}

impl std::fmt::Display for OtherError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OtherError::Unknown => write!(f, "unknown error"),
        }
    }
}

impl std::error::Error for OtherError {}

impl From<MyError> for OtherError {
    fn from(e: MyError) -> Self {
        match e {
            MyError::EnvironmentVariableNotFound => OtherError::Unknown,
            MyError::IOError(_) => OtherError::Unknown,
            MyError::Other => OtherError::Unknown,
        }
    }
}

fn render() -> Result<String, MyError> {
    let file = std::env::var("MARKDOWN")?;
    let source = read_to_string(file)?;
    Ok(source)
}

fn show_error() -> Result<(), MyError> {
    render()?;
    Ok(())
}

fn handle_error() -> Result<(), OtherError> {
    render()?;
    Ok(())
}

fn main() -> Result<(), MyError> {
    let html = render();
    println!("{html:?}");

    let r = show_error();
    println!("{r:?}");

    let r = handle_error();
    println!("{r:?}");

    Ok(())
}
