use plotters::prelude::*;
use sqlx::SqlitePool;

pub async fn plot_energy_usage(db: &SqlitePool) {
    let transactions: Vec<(String, f64)> = sqlx::query_as("SELECT timestamp, energy FROM transactions ORDER BY timestamp")
        .fetch_all(db)
        .await
        .unwrap();

    let root = BitMapBackend::new("data/plots/energy_usage.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root)
        .caption("Energy Usage Over Time", ("sans-serif", 20).into_font())
        .x_label_area_size(30)
        .y_label_area_size(40)
        .build_cartesian_2d(0..transactions.len(), 0f64..10f64)
        .unwrap();

    chart.configure_mesh().draw().unwrap();
    chart.draw_series(LineSeries::new(
        transactions.iter().enumerate().map(|(i, (_, e))| (i, *e)),
        &RED,
    )).unwrap();
}