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
                .help("Run in server mode (SSH server on port 22) [default]")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("local")
                .short('l')
                .long("local")
                .help("Run in local mode (direct terminal UI)")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let server_mode = matches.get_flag("server");
    let local_mode = matches.get_flag("local");

    if server_mode && local_mode {
        eprintln!("Error: Cannot specify both --server and --local flags");
        std::process::exit(1);
    }

    if local_mode {
        let local_tui = LocalTuiRunner::new();
        local_tui.run().await
    } else {
        let mut server = AppServer::new();
        server.run().await
    }
}
