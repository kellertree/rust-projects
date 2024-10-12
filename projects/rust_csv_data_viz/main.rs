use csv::ReaderBuilder;
use chrono::NaiveDate;
use plotters::prelude::*;
use std::fs::File;
use std::error::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Record {
    date: String,
    value: f64,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read CSV file
    let file_path = "data.csv"; // replace with file input mechanism
    let mut rdr = ReaderBuilder::new().has_headers(true).from_path(file_path)?;

    // Parse data
    let mut dates: Vec<NaiveDate> = Vec::new();
    let mut values: Vec<f64> = Vec::new();

    for result in rdr.deserialize() {
        let record: Record = result?;
        let date = NaiveDate::parse_from_str(&record.date, "%Y-%m-%d")?;
        dates.push(date);
        values.push(record.value);
    }

    // Create plot
    let root = BitMapBackend::new("output.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Time Series Plot", ("sans-serif", 40).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(40)
        .build_cartesian_2d(dates[0]..dates[dates.len() - 1], 0.0..*values.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap())?;

    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(
        dates.iter().zip(values.iter()).map(|(d, v)| (*d, *v)),
        &BLUE,
    ))?;

    root.present()?;

    println!("Plot saved to output.png");

    Ok(())
}
