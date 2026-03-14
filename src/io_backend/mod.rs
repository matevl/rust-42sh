use std::io::{self, Cursor, Read};
use std::fs::File;

#[derive(Debug)]
pub enum InputType {
    String(String),
    File(String),
    Stdin,
}

pub struct IoBackend {
    reader: Box<dyn Read>,
}

impl IoBackend {
    pub fn new(input_type: InputType) -> io::Result<Self> {
        let reader: Box<dyn Read> = match input_type {
            InputType::String(s) => Box::new(Cursor::new(s.into_bytes())),
            InputType::File(path) => Box::new(File::open(path)?),
            InputType::Stdin => Box::new(io::stdin()),
        };
        Ok(IoBackend { reader })
    }

    /// Reads a single character from the input stream.
    pub fn read_char(&mut self) -> Option<char> {
        let mut buffer = [0; 1];
        if let Ok(()) = self.reader.read_exact(&mut buffer) {
             Some(buffer[0] as char)
        } else {
             None
        }
    }
}

impl Iterator for IoBackend {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.read_char()
    }
}
