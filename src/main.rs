extern crate serde_json;

use csv::Reader;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::process;

use serde_json::json;
use serde_json::{Map, Value};

type Row = Map<String, Value>;
type Rows = Vec<Row>;

fn get_dynamic_value(string: String) -> Result<Value, Box<dyn Error>> {
    if let Ok(integer) = string.parse::<i32>() {
        Ok(json!(integer))
    } else if let Ok(boolean) = string.parse::<bool>() {
        Ok(json!(boolean))
    } else {
        Ok(json!(string))
    }
}

fn read_rows() -> Result<Rows, Box<dyn Error>> {
    let mut rdr = Reader::from_reader(io::stdin());
    let headers = rdr.headers().cloned()?;
    let mut rows: Rows = Vec::new();

    for record in rdr.records() {
        let mut row: Row = Map::new();
        let cells = record?;

        for (i, cell) in cells.iter().enumerate() {
            let header = headers.get(i).unwrap().to_string();
            let value = get_dynamic_value(cell.to_string())?;

            row.insert(header, value);
        }

        rows.push(row);
    }

    Ok(rows)
}

fn to_json(rows: Rows) -> Result<String, Box<dyn Error>> {
    Ok(serde_json::to_string_pretty(&rows)?)
}

fn write_json(string: String) -> Result<(), Box<dyn Error>> {
    let mut file = File::create("out.json")?;

    file.write_all(string.as_bytes())?;

    Ok(())
}

fn main() {
    if let Err(err) = read_rows().and_then(to_json).and_then(write_json) {
        eprintln!("{:?}", err);
        process::exit(1);
    }
}
