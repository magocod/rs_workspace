use tokio::signal::unix::{signal, SignalKind};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sigint = signal(SignalKind::interrupt())?;

    match sigint.recv().await {
        Some(()) => println!("Received SIGINT signal"),
        None => eprintln!("Stream terminated before receiving SIGINT signal"),
    }

    for num in 0..10000 {
        println!("{}", num)
    }

    Ok(())
}
