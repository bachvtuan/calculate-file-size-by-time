
# Folder Size Analyzer

This Rust program analyzes the file sizes within one or more specified folders, summarizing the total file size by year or by month for a given year. It provides a quick way to see storage usage trends and identify storage-heavy periods in a file system.

## Features

- **Analyze Multiple Folders**: Calculate file sizes across multiple folders.
- **Size by Year**: Summarize file sizes by year if no specific year is provided.
- **Size by Month**: Specify a year to break down file sizes by month within that year.
- **Gigabyte Conversion**: Outputs file sizes in gigabytes for easy readability.
- **Parallel Processing**: Uses multithreading for faster analysis on large datasets.

## Prerequisites

- **Rust**: Ensure that [Rust](https://www.rust-lang.org/tools/install) is installed on your system.

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/folder-size-analyzer.git
   cd folder-size-analyzer
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

Run the program with the following syntax:

```bash
cargo run -- <folder_path1> [<folder_path2> ...] [year]
```

- **folder_path**: Path(s) to one or more folders you want to analyze.
- **year** (optional): A specific year to analyze monthly file sizes within that year.



This will generate a binary file in the `target/release` directory.

## Running the Binary

Once built, you can run the program directly using the compiled binary instead of `cargo run`. Navigate to the `target/release` directory and run the binary as follows:

```bash
./target/release/folder-size-analyzer <folder_path1> [<folder_path2> ...] [year]
```

- **folder_path**: Path(s) to one or more folders you want to analyze.
- **year** (optional): A specific year to analyze monthly file sizes within that year.


### Examples

1. **Analyze by Year Across Multiple Folders**:
   ```bash
   cargo run -- /path/to/folder1 /path/to/folder2
   ```
   This command will calculate the total file size by year for each folder.

2. **Analyze by Month in a Specific Year Across Multiple Folders**:
   ```bash
   cargo run -- /path/to/folder1 /path/to/folder2 2022
   ```
   This command will calculate the total file size for each month in the year 2022 for each folder.

### Sample Output

#### Yearly Summary
```plaintext
File sizes by year in folder '/path/to/folder1':
Year: 2020, Total Size: 4.75 GB
Year: 2021, Total Size: 2.30 GB

File sizes by year in folder '/path/to/folder2':
Year: 2020, Total Size: 5.50 GB
Year: 2021, Total Size: 1.80 GB
```

#### Monthly Summary for a Specific Year
```plaintext
File sizes for year 2022 in folder '/path/to/folder1':
Month: 1, Total Size: 0.25 GB
Month: 2, Total Size: 0.75 GB
...
```

## Additional Notes

- **Threshold**: By default, the program filters out any yearly totals under 100 MB.
- **Parallel Processing**: The program uses Rayon for parallel processing, which improves performance on larger directories.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.
