use clap::{Parser, Subcommand};
use sqlx::SqlitePool;

#[derive(Parser)]
#[command(name = "ev_status")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Status,
    Plot,
}

pub async fn run(db: SqlitePool) {
    let cli = Cli::parse();

    match cli.command {
        Commands::Status => {
            let txs: Vec<(String, f64, f64)> = sqlx::query_as("SELECT charger_id, energy, cost FROM transactions")
                .fetch_all(&db)
                .await
                .unwrap();
            for (id, energy, cost) in txs {
                println!("Charger: {}, Energy: {} kWh, Cost: ${}", id, energy, cost);
            }
        }
        Commands::Plot => {
            crate::analytics::plot::plot_energy_usage(&db).await;
            println!("Plot saved to data/plots/energy_usage.png");
        }
    }
}