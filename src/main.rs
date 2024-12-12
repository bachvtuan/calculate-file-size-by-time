use std::collections::HashMap;
use std::env;
use std::fs;
use std::sync::Mutex;
use walkdir::WalkDir;
use chrono::{DateTime, Utc, Datelike};
use rayon::prelude::*;

fn main() {
    // Get the folder paths and optional year from the command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <folder_path1> [<folder_path2> ...] [year]", args[0]);
        std::process::exit(1);
    }

    // Separate folder paths and optional year argument
    let (folder_paths, year_arg) = args[1..]
        .split_last()
        .map(|(last, rest)| match last.parse::<i32>() {
            Ok(year) => (rest.to_vec(), Some(year)), // If last arg is a valid year, treat as year
            Err(_) => (args[1..].to_vec(), None),    // Otherwise, treat all args as paths
        })
        .unwrap();

    if let Some(year) = year_arg {
        // Calculate and display file sizes by month for the specified year, per folder
        for folder_path in &folder_paths {
            match calculate_file_sizes(folder_path, Some(year)) {
                Ok(size_by_times) => {
                    println!("\nFile sizes for year {} in folder '{}':", year, folder_path);

                    // Sort months in ascending order
                    let mut sorted_months: Vec<_> = size_by_times.into_iter().collect();
                    sorted_months.sort_by_key(|&(month, _)| month);

                    for (month, size_bytes) in sorted_months {
                        let size_gb = size_bytes as f64 / 1_073_741_824.0; // Convert bytes to GB
                        println!("Month: {}, Total Size: {:.2} GB", month, size_gb);
                    }
                }
                Err(e) => eprintln!("Error in folder '{}': {}", folder_path, e),
            }
        }
    } else {
        // Calculate and display file sizes by year, per folder
        for folder_path in &folder_paths {
            match calculate_file_sizes(folder_path, None) {
                Ok(size_by_year) => {
                    println!("\nFile sizes by year in folder '{}':", folder_path);
                    let threshold = 100 * 1_048_576; // 100 MB in bytes
                    let mut filtered_years: Vec<_> = size_by_year
                        .into_iter()
                        .filter(|&(_, size_bytes)| size_bytes > threshold)
                        .collect();
                    filtered_years.sort_by_key(|&(year, _)| year);

                    for (year, size_bytes) in filtered_years {
                        let size_gb = size_bytes as f64 / 1_073_741_824.0; // Convert bytes to GB
                        println!("Year: {}, Total Size: {:.2} GB", year, size_gb);
                    }
                }
                Err(e) => eprintln!("Error in folder '{}': {}", folder_path, e),
            }
        }
    }
}


fn calculate_file_sizes(folder_path: &str, year: Option<i32>) -> Result<HashMap<i32, u64>, Box<dyn std::error::Error>> {
    let size_by_times = Mutex::new(HashMap::new());
    let working_year = year.unwrap_or(0); //Default value is 0

    WalkDir::new(folder_path)
        .into_iter()
        .par_bridge()
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_file())
        .for_each(|entry| {
            if let Ok(metadata) = fs::metadata(entry.path()) {
                let size = metadata.len();

                if let Ok(modified) = metadata.modified() {
                    
                    let datetime: DateTime<Utc> = modified.into();
                    let m_year: i32 = datetime.year();
                    let mut key: i32 = 0;
                    if working_year > 0{
                        if m_year == working_year {
                            key =  datetime.month() as i32;
                        }
                    }else{
                        key = m_year;
                    }
                    if key > 0{
                        // Accumulate sizes by month in a thread-safe way
                        let mut size_by_times = size_by_times.lock().unwrap();
                        *size_by_times.entry(key).or_insert(0) += size;
                    }

                }
            }
        });

    Ok(size_by_times.into_inner().unwrap())
}
