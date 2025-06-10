use std::time::Duration;
use std::time::Instant;
use futures::future;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let now = Instant::now();

    let future_1 = one_sec_nya();
    let future_2 = zero_five_sec_nya();
    let future_3 = zero_three_sec_nya();

    future::join3(
        future_1,
        future_2,
        future_3
    ).await;

    println!("{:?}", now.elapsed())
}

async fn one_sec_nya() {
    println!("1");
    sleep(
        Duration::from_secs_f64(1.0)
    ).await;
    println!("にゃー");
}

async fn zero_five_sec_nya() {
    println!("0.5");
    sleep(
        Duration::from_secs_f64(0.5)
    ).await;
    println!("にゃにゃー");
}

async fn zero_three_sec_nya() {
    println!("0.3");
    sleep(
        Duration::from_secs_f64(0.3)
    ).await;
    println!("にゃにゃにゃー");
}
