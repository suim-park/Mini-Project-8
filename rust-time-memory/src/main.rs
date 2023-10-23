use std::time::Instant;
use psutil::process::Process;
use csv::ReaderBuilder;
use std::error::Error;
use std::fs::File;

fn calculate_time_memory(path: &str) -> (i64, f64) {
    // Record the start time
    let start_time = Instant::now();

    // Measure the initial resource usage
    let process = Process::new(std::process::id());
    let start_mem_usage = process.as_ref().expect("Failed to create process").memory_info().expect("Failed to get memory info").rss();

    // Calculate the average
    let _average_result = average(path);

    // Record the end time
    let end_time = Instant::now();

    // Measure the final resource usage
    let end_mem_usage = process.as_ref().expect("Failed to create process").memory_info().expect("Failed to get memory info").rss();

    // Calculate the elapsed time
    let elapsed_time = end_time.duration_since(start_time).as_secs_f64();

    println!("Rust-Elapsed Time: {:.7} seconds", elapsed_time);
    println!("Rust-Memory Usage Change: {} kilobytes", end_mem_usage - start_mem_usage);

    (end_mem_usage as i64, elapsed_time)
}

struct BirthData {
    weight: f64,
    // Add other fields from your CSV if needed
}

fn average(path: &str) -> Result<f64, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut rdr = ReaderBuilder::new().delimiter(b',').from_reader(file);

    let mut sum = 0.0;
    let mut count = 0;

    for result in rdr.records() {
        let record = result?;
        if let Some(weight) = record.get(7) {
            if let Ok(parsed_weight) = weight.parse::<f64>() {
                sum += parsed_weight;
                count += 1;
            }
        }
    }

    if count == 0 {
        return Err("No data found".into());
    }

    Ok(sum / count as f64)
}

fn main() {
    let path = "/workspaces/Mini-Project-8/births14.csv"; // Updated with the actual CSV file path

    match average(path) {
        Ok(avg) => println!("Average weight: {:.5}", avg),
        Err(err) => eprintln!("Error: {}", err),
    }

    let (end_mem_usage, elapsed_time) = calculate_time_memory(path);

    println!("Final Memory Usage: {} kilobytes", end_mem_usage);
    println!("Total Elapsed Time: {:.7} seconds", elapsed_time);
}
