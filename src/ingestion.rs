use crate::model::Record;
use arrow::array::{ArrayRef, Float64Array, Int32Array, StringArray};
use arrow::record_batch::RecordBatch;
use arrow_schema::{DataType, Field, Schema};
use parquet::arrow::arrow_writer::ArrowWriter;
use parquet::file::properties::WriterProperties;
use polars::prelude::*;
use std::collections::HashMap;
use std::error::Error;
use std::fs::{self, create_dir_all};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::sync::Arc;

pub fn process_json_to_parquet(output_dir: &Path) -> Result<(), Box<dyn Error>> {
    let schema = Arc::new(Schema::new(vec![
        Field::new("version", DataType::Utf8, false),
        Field::new("name", DataType::Utf8, false),
        Field::new("uuid", DataType::Utf8, false),
        Field::new("latitude", DataType::Float64, false),
        Field::new("longitude", DataType::Float64, false),
        Field::new("altitude", DataType::Float64, false),
        Field::new("course", DataType::Float64, false),
        Field::new("speed", DataType::Int32, false),
        Field::new("timestamp", DataType::Utf8, false),
    ]));

    let records: Vec<Record> = BufReader::new(std::io::stdin())
        .lines()
        .filter_map(|line| {
            let line = line.ok()?;
            serde_json::from_str::<Record>(&line).ok()
        })
        .collect();

    if records.is_empty() {
        return Err("No valid records found in input".into());
    }

    // Group records by partition keys
    let mut partitions: HashMap<String, Vec<Record>> = HashMap::new();

    for record in records {
        let partition_key = format!(
            "name={}/year={}/month={}/day={}",
            record.name,
            record.year(),
            record.month(),
            record.day()
        );
        partitions.entry(partition_key).or_default().push(record);
    }

    // Write each partition to a separate Parquet file
    for (partition_path, partition_records) in partitions {
        let full_path = output_dir.join(&partition_path);
        create_dir_all(&full_path)?;

        let file_path = full_path.join("data.parquet");
        let file = fs::File::create(&file_path)?;

        // Convert records to Arrow arrays
        let versions: Vec<&str> = partition_records
            .iter()
            .map(|r| r.version.as_str())
            .collect();
        let names: Vec<&str> = partition_records.iter().map(|r| r.name.as_str()).collect();
        let uuids: Vec<&str> = partition_records.iter().map(|r| r.uuid.as_str()).collect();
        let latitudes: Vec<f64> = partition_records.iter().map(|r| r.latitude).collect();
        let longitudes: Vec<f64> = partition_records.iter().map(|r| r.longitude).collect();
        let altitudes: Vec<f64> = partition_records.iter().map(|r| r.altitude).collect();
        let courses: Vec<f64> = partition_records.iter().map(|r| r.course).collect();
        let speeds: Vec<i32> = partition_records.iter().map(|r| r.speed).collect();
        let timestamps: Vec<String> = partition_records
            .iter()
            .map(|r| r.timestamp.to_rfc3339())
            .collect();

        let arrays: Vec<ArrayRef> = vec![
            Arc::new(StringArray::from(versions)),
            Arc::new(StringArray::from(names)),
            Arc::new(StringArray::from(uuids)),
            Arc::new(Float64Array::from(latitudes)),
            Arc::new(Float64Array::from(longitudes)),
            Arc::new(Float64Array::from(altitudes)),
            Arc::new(Float64Array::from(courses)),
            Arc::new(Int32Array::from(speeds)),
            Arc::new(StringArray::from(timestamps.clone())),
        ];

        let batch = RecordBatch::try_new(schema.clone(), arrays)?;
        let props = WriterProperties::builder().build();
        let mut writer = ArrowWriter::try_new(file, schema.clone(), Some(props))?;

        writer.write(&batch)?;
        writer.close()?;
    }

    Ok(())
}
