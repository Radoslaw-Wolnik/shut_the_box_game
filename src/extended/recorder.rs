use std::fs::File;
use std::io::{BufWriter, Write};
use crate::extended::GameResult;

pub struct ExtendedRecorder {
    writer: BufWriter<File>,
    sample_rate: u32,
    current: u32,
}

impl ExtendedRecorder {
    pub fn new(path: &str, sample_rate: u32) -> Self {
        let writer = BufWriter::new(File::create(path).expect("Failed to open extended file"));
        ExtendedRecorder { writer, sample_rate, current: 0 }
    }

    pub fn record(&mut self, result: &GameResult) {
        self.current += 1;
        if self.current % self.sample_rate == 0 {
            writeln!(self.writer, "{}", serde_json::to_string(result).unwrap()).ok();
        }
    }

    pub fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}