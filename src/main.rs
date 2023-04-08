use std::env;
use csv;
use serde::Deserialize;
use std::collections::HashMap;
use std::ffi::OsString;

#[derive(Debug, Deserialize)]
struct Row {
    id: String,
    group: String,
    value: f32
}

fn file_reader(file: OsString) -> HashMap<String,f32> {
    let mut reader = csv::Reader::from_path(file).expect("Could not read csv file.");

    let mut values_by_group = HashMap::new();

    for result in reader.deserialize() {
        let record: Row = result.expect("Could not read row");
        *values_by_group.entry(record.group).or_insert(0.0) += record.value;
    }
    values_by_group
}


fn main() {
    let file_path = env::args_os().nth(1).expect("No arguments provided");
    let values_by_group = file_reader(file_path);
    println!("{:?}", values_by_group);
}
