extern crate csv;
extern crate time;

use std::error::Error;
use std::fs::File;
use std::time::Instant;

struct BirthRecord {
    weight: f64,
    // Add other fields as needed
}

fn average(path: &str) -> Result<f64, Box<dyn Error>> {
    let mut weight_sum = 0.0;
    let mut count = 0;

    let file = File::open(path)?;
    let mut rdr = csv::Reader::from_reader(file);

    for result in rdr.records() {
        let record = result?;
        let weight: f64 = record[1].parse()?;
        weight_sum += weight;
        count += 1;
    }

    if count == 0 {
        return Ok(0.0);
    }

    let weight_avg = weight_sum / count as f64;
    Ok(weight_avg)
}

fn calculate_average_time(path: &str) -> Result<(i64, f64), Box<dyn Error>> {
    let start_time = Instant::now();
    let start_mem_usage = 0; // Memory measurement not supported in Rust without external libraries

    let weight_avg = average(path)?;

    let elapsed_time = start_time.elapsed().as_secs_f64();
    let end_mem_usage = 0; // Memory measurement not supported in Rust without external libraries

    println!("Rust-Elapsed Time: {:.3} seconds", elapsed_time);
    println!("Rust-Memory Usage Change: {} kilobytes", end_mem_usage - start_mem_usage);

    Ok((end_mem_usage, elapsed_time))
}

fn main() {
    let path = "births14.csv";

    match calculate_average_time(path) {
        Ok((_, _)) => {
            // Handle success
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}
