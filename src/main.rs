use clap::{Arg, Command};

mod app;
mod local_tui;
mod pages;
mod server;

use local_tui::LocalTuiRunner;
use server::AppServer;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let matches = Command::new("portfolio-v2")
        .version("0.1.0")
        .about("A terminal-based portfolio application")
        .arg(
            Arg::new("server")
                .short('s')
                .long("server")
                .help("Run in server mode (SSH server on port 22)")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let server_mode = matches.get_flag("server");

    if server_mode {
        let mut server = AppServer::new();
        server.run().await
    } else {
        let local_tui = LocalTuiRunner::new();
        local_tui.run().await
    }
}
