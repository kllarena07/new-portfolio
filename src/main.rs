mod app;
mod pages;
mod server;

use server::AppServer;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let mut server = AppServer::new();
    server.run().await
}
