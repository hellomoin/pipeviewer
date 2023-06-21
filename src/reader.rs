use crate::CHUNK_SIZE;

use std::{io::{self, BufReader, Read, Result}, ops::Bound};
use std::fs::File;
use std::sync::{Arc, Mutex};

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

        pub fn read(&mut self, quit: Arc<Mutex<bool>>) -> Result<()> {
            let mut buffer = [0; CHUNK_SIZE];
        
            loop {
                let num_read = match self.reader.read(&mut buffer) {
                    Ok(0) => break,
                    Ok(x) => x,
                    Err(_) => break,
                };

                // Vec::from(&buffer[..num_read])
            }

            let mut quit = quit.lock().unwrap();
            *quit = true;

            Ok(())
        }

}

pub fn read_loop(infile: &str, quit: Arc<Mutex<bool>>) -> Result<()> {
    let mut reader = match Reader::new(infile) {
        Ok(x) => x,
        Err(x) => {
            println!("failed to open the file");
            return Err(x)
        }
    };

    reader.read(quit)?;

    Ok(())
}