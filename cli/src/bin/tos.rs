use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Semaphore;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let semaphore = Arc::new(Semaphore::new(2));
    let mut join_handles = Vec::new();

    for i in 0..5 {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        join_handles.push(tokio::spawn(async move {
            println!("task {i}");
            sleep(Duration::from_millis(1000)).await;
            drop(permit);
        }));
    }

    for handle in join_handles {
        handle.await.unwrap();
    }
}
