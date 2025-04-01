use clap::{Parser, Subcommand};
use sqlx::SqlitePool;
use crate::cms::db::Transaction;

#[derive(Parser)]
#[command(name = "ev_status")]
#[command(subcommand_required = false)] 
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Status,
    Plot,
}

pub async fn run(db: SqlitePool) {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Status) => {
            let txs: Vec<Transaction> = sqlx::query_as("SELECT id, charger_id, energy, cost, timestamp FROM transactions")
                .fetch_all(&db)
                .await
                .unwrap_or_else(|e| {
                    println!("Error fetching transactions: {}", e);
                    Vec::new()
                });
            if txs.is_empty() {
                println!("No transactions found.");
            } else {
                for tx in txs {
                    println!(
                        "Transaction ID: {}, Charger: {}, Energy: {} kWh, Cost: ${}, Timestamp: {}",
                        tx.id, tx.charger_id, tx.energy, tx.cost, tx.timestamp
                    );
                }
            }
        }
        Some(Commands::Plot) => {
            crate::analytics::plot::plot_energy_usage(&db).await;
            println!("Plot saved to data/plots/energy_usage.png");
        }
        None => {
            // No-op here; main.rs handles the default CMS/charger logic
        }
    }
}

