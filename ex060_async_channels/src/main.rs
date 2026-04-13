use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

async fn producer(id: u32, tx: mpsc::Sender<String>) {
    for i in 0..5 {
        let msg = format!("Producer {} send item {}", id, i);
        let _ = tx.send(msg).await;
        sleep(Duration::from_millis(100)).await
    }
}

async fn consumer(mut rx: mpsc::Receiver<String>) {
    while let Some(msg) = rx.recv().await {
        println!("Consumer received: {}", msg);
        sleep(Duration::from_millis(100)).await;
    }
}

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel(100);

    for i in 0..3 {
        let tx = tx.clone();
        tokio::spawn(async move {
            producer(i, tx).await;
        });
    }

    drop(tx);

    consumer(rx).await;
}
