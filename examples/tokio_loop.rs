#[tokio::main(flavor = "current_thread")]
async fn main() {
    tokio::spawn(async { std::thread::sleep(std::time::Duration::from_secs(10)) });
    tokio::spawn(async {
        println!("it is my time");
    });
    println!("start to sleep");
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
}
