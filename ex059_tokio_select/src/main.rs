use tokio::time::{sleep, Duration};

async fn task_one() {
    sleep(Duration::from_secs(2)).await;
    println!("Task one completed!!!");    
}

async fn task_two() {
    sleep(Duration::from_secs(1)).await;
    println!("Task two completed!!!")
}

#[tokio::main]
async fn main() {
    tokio::select! {
        _ = task_one() => println!("Task one won!!!"),
        _ = task_two() => println!("Task two won!!!"),
    }
}
