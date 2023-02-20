

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tokio::spawn(async {
        let mut n = 1;
        loop {
            n += 1;
            if n % 10001659 == 0 {
                println!("reach 10001659")
            }
        }
    });
    tokio::spawn(async {
        println!("it is my time");
    });
    println!("start to sleep");
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
}
