mod charge_point;
mod cms;
mod ocpp;
mod analytics;
mod hardware;
mod status;

use cms::{db::init_db, server::CMS};
use charge_point::ws_client;
use clap::Parser;

#[tokio::main]
async fn main() {
    let cli = status::cli::Cli::parse();

    let db_url = "sqlite:data/chargers.db";
    let db = init_db(db_url).await;

    match cli.command {
        Some(status::cli::Commands::Status) => {
            status::cli::run(db).await;
        }
        Some(status::cli::Commands::Plot) => {
            status::cli::run(db).await;
        }
        None => {
            // Default behavior: Run CMS and chargers (when no CLI command is provided)
            let cms = CMS::new(0.25, db.clone());
            tokio::spawn(async move {
                cms.start("127.0.0.1:8081").await;
            });

            tokio::spawn(async {
                ws_client::connect("CP_001", 7.0, "ws://127.0.0.1:8081").await;
            });
            tokio::spawn(async {
                ws_client::connect("CP_002", 11.0, "ws://127.0.0.1:8081").await;
            });

            tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
            status::cli::run(db).await;
        }
    }
}

