use snac3::incoming::http::{HttpServer, HttpServerConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let server_config = HttpServerConfig { port: "3000" };
    let http_server = HttpServer::new(server_config).await?;
    http_server.run().await
}
