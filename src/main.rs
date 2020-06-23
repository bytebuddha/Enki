extern crate enki;

#[tokio::main]
async fn main() -> enki::core::Result<()> {
    enki::run().await
}
