use reqwest;
#[tokio::main]
async fn main() {
    // chaining .await will yield our query result
    let result = reqwest::get("127.0.0.1:7879").await;
    println!("{:?}", result);
}
