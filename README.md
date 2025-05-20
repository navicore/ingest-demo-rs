# Ingest Demo RS

A Rust CLI tool for generating random JSON data and converting JSON to partitioned Parquet files.

## Features

- Generate random JSON records based on example templates
- Convert JSON input to partitioned Parquet files (by name, year, month, day)

## Installation

```bash
cargo build --release
```

The binary will be located at `target/release/ingest-demo`.

## Usage

### Generate random JSON records

```bash
# Generate 100 random JSON records
./target/release/ingest-demo generate --count 100 > output.json

# Use a custom example template
./target/release/ingest-demo generate --count 50 --example path/to/custom_example.json > output.json
```

### Convert JSON to Parquet

```bash
# Convert JSON from a file to Parquet (using default output directory 'data')
cat output.json | ./target/release/ingest-demo convert

# Specify a custom output directory
cat output.json | ./target/release/ingest-demo convert --output custom_data_dir
```

### Complete Pipeline Example

```bash
# Generate 1000 records and immediately convert to Parquet
./target/release/ingest-demo generate --count 1000 | ./target/release/ingest-demo convert --output data_dir
```

## Partitioning

The convert command creates a partitioned directory structure for efficient querying:

```
data/
├── name=Boat 1/
│   └── year=2025/
│       └── month=04/
│           └── day=16/
│               └── data.parquet
├── name=Boat 2/
│   └── ...
...
```

This partitioning allows efficient querying by name, year, month, and day.
