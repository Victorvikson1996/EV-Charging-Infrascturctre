mod charge_point;
mod cms;
mod ocpp;
mod analytics;
mod hardware;
mod status;

use cms::{db::init_db, server::CMS};
use charge_point::ws_client;

#[tokio::main]
async fn main() {
    let db_url = "sqlite:data/chargers.db";
    let db = init_db(db_url).await;

    // Start CMS
    let cms = CMS::new(0.25, db.clone());
    tokio::spawn(async move {
        cms.start("127.0.0.1:8080").await;
    });

    // Start multiple chargers
    tokio::spawn(async {
        ws_client::connect("CP_001", 7.0, "ws://127.0.0.1:8080").await;
    });
    tokio::spawn(async {
        ws_client::connect("CP_002", 11.0, "ws://127.0.0.1:8080").await;
    });

    // Status tool
    tokio::time::sleep(tokio::time::Duration::from_secs(30)).await; // Let chargers run
    status::cli::run(db).await;
}