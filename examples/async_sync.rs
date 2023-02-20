#[tokio::main(flavor = "current_thread")]
async fn main() {
    tokio::spawn(async {
        println!("async sleep wait");
        tokio::spawn(async {
            println!("it is my time");
        });
        std::thread::sleep(std::time::Duration::from_secs(5));
    });

    println!("start to sleep");

    tokio::time::sleep(std::time::Duration::from_secs(10)).await;
}
