use std::{fs::File};
use csv::Writer;
// use serde::Serialize;
use super::result::GameResult;

pub struct Recorder {
    writer: Writer<File>, // Option<csv::Writer<File>>,
}

impl Recorder {
    pub fn new(path: &str) -> Self {
        let mut wtr = Writer::from_path(path).expect("Failed to open base CSV");
        wtr.write_record(&["id","sum_a","sum_b","a0","a1","a2","a3","a4","b0","b1","b2","b3","b4"]).unwrap();
        Recorder { writer: wtr }
    }

    pub fn record(&mut self, result: &GameResult) {
        self.writer.serialize(result).unwrap();
    }

    pub fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}