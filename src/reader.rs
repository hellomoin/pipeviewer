use crate::CHUNK_SIZE;

use std::{io::{self, BufReader, Read, Result}};
use std::fs::File;
use std::sync::mpsc::Sender;

pub struct Reader {
    reader: Box<dyn Read>,
}

impl Reader {
        pub fn new(infile: &str) -> Result<Self> {
            let reader: Box<dyn Read>  = if !infile.is_empty() {
                Box::new(BufReader::new(File::open(infile)?))
            } else {
                Box::new(BufReader::new(io::stdin()))
            };

            Ok(Self {
                reader
            })
        }

        pub fn read(&mut self, stats_tx: Sender<Vec<u8>>) -> Result<()> {
            let mut buffer = [0; CHUNK_SIZE];
        
            loop {
                let num_read = match self.reader.read(&mut buffer) {
                    Ok(0) => break,
                    Ok(x) => x,
                    Err(_) => break,
                };

                if stats_tx.send(Vec::from(&buffer[..num_read])).is_err() {
                    break;
                }
            }
            let _ = stats_tx.send(Vec::new()); // send empty buffer
            Ok(())
        }

}

pub fn read_loop(infile: &str, stats_tx: Sender<Vec<u8>>) -> Result<()> {
    let mut reader = match Reader::new(infile) {
        Ok(x) => x,
        Err(x) => {
            println!("failed to open the file");
            return Err(x)
        }
    };

    reader.read(stats_tx)?;

    Ok(())
}