#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bytes = reqwest::Client::new()
        .post("http://127.0.0.1:3030/sign")
        .body(r#"{"id": 1, "jsonrpc": "2.0", "method": "sign", "params": [5, 60, 241, 0, 9, 54]}"#)
        .header("Content-Type", "application/json")
        .send()
        .await?
        .bytes()
        .await?;
    println!("{:#?}", bytes);
    Ok(())
}