use tokio;
use futures::join;

async fn step1() -> String {
    "Hello".to_string()
} 

async fn step2(msg: String) -> String {
    format!("{}, World", msg)
}

async fn chained() {
    let message = step2(step1().await).await;
    println!("{}", message);
}

async fn joined() {
    let (a,b) = join!(step1(), step2("Hello".to_string()));
    println!("Results: {}, {}", a, b);
}

#[tokio::main]
async fn main() {
    chained().await;
    joined().await;
}
