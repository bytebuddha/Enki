use serde_json::json;

use std::io;

use crate::xi::TestClient;
use enki::xi::Response;
use enki::xi::{ClientExt, Message};

#[tokio::test]
async fn new_view() -> io::Result<()> {
    let mut client = TestClient::embeded().await?;
    let expected = Message::Response(Response {
        id: 0,
        result: Ok(json!("view-id-1")),
    });
    client.new_view(None).await?;
    client.check_responses(None, expected).await?;
    Ok(())
}

#[tokio::test]
async fn new_view_from_file() -> io::Result<()> {
    let mut client = TestClient::embeded().await?;
    let expected = Message::Response(Response {
        id: 0,
        result: Ok(json!("view-id-1")),
    });
    client.new_view(Some("Cargo.toml".into())).await?;
    client.check_responses(None, expected).await?;
    Ok(())
}
