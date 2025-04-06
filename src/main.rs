mod fuse;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let res = fuse::example_usage().await;
    println!("Result: {:?}", res);
    
    Ok(())
}
