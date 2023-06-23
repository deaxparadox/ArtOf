use mini_redis::{client, Result};
use tokio;

async fn say_world() {
    println!("world");
}


async fn hello_world() {
    let op = say_world();

    // This println! comes first
    println!("hello");

    // Calling `.await` on `op` start executing `say world`.
    op.await;
}



#[tokio::main]
async fn main() -> Result<()> {
    // Open an connection to the mini-redis address.
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Set the key "hello" with value "world"
    client.set("hello", "world".into()).await?;

    // Get key "hello"
    let result = client.get("hello").await?;

    println!("got value from the server; result={:?}", result);

    hello_world().await;
    
    Ok(())
}