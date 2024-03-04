use std::fs::File;
use std::io::{stdin, BufRead, BufReader, Result};
use std::path::PathBuf;

use anyhow::Context;

pub fn get_reader(input_path: Option<PathBuf>) -> Result<Box<dyn BufRead>> {
    let reader: Box<dyn BufRead> = match &input_path {
        None => Box::new(BufReader::new(stdin())),
        Some(file_path) => Box::new(BufReader::new(
            File::open(file_path)
                .with_context(|| format!("Error reading file {}", file_path.display()))
                .unwrap(),
        )),
    };

    Ok(reader)
}
