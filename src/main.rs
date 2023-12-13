use serde::Deserialize;
use std::{env, error::Error, ffi::OsString, io, process};

fn write_run() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let mut wtr = csv::Writer::from_path(file_path)?;
    wtr.write_record(&["a", "b"])?;
    wtr.write_record(&["1", "2"])?;
    wtr.flush()?;
    Ok(())
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Record {
    latitude: f64,
    longitude: f64,
    #[serde(deserialize_with = "csv::invalid_option")]
    population: Option<u64>,
    city: String,
    state: String,
}

fn read_run() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(file_path)?;

    for result in rdr.deserialize::<Record>() {
        let record = result?;
        println!("{:?}", record);
    }

    Ok(())
}

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn main() {
    if let Err(err) = write_run() {
        println!("{}", err);
        process::exit(1);
    }
}
