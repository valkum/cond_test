use anyhow::Result as AnyResult;
use mini_redis::{Result};

#[tokio::main]
pub async fn main() -> Result<()> {
    println!("{}", hello_world()?);
    Ok(())
}

fn hello_world() -> AnyResult<String> {
    Ok("Hello, world!".to_string())
}
