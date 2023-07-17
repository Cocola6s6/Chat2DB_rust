use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().unwrap();

    println!("Hello, world!");
}
