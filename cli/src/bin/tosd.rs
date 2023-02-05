use tokio::signal;

#[tokio::main]
async fn main() {
    // ... spawn application as separate task ...

    match signal::ctrl_c().await {
        Ok(()) => {
            eprintln!("listen for shutdown signal: ctrl_c");
        }
        Err(err) => {
            eprintln!("Unable to listen for shutdown signal: {err}");
        }
    }
}
