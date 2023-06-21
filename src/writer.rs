use std::{io::{self, ErrorKind,BufWriter, Write, Result}};
use std::fs::File;
use std::sync::{Arc, Mutex};

pub struct Writer {
    writer: Box<dyn Write>,
}

impl Writer {
    pub fn new(outfile: &str) -> Result<Self> {
        let writer: Box<dyn Write>  = if !outfile.is_empty() {
            Box::new(BufWriter::new(File::create(outfile)?))
        } else {
            Box::new(BufWriter::new(io::stdout()))
        };

        Ok(Self {
            writer
        })
    }

    pub fn write(&mut self, quit: Arc<Mutex<bool>>) -> Result<()> {
        loop {
            let buffer: Vec<u8> = Vec::new();

            // Need this block to unlock quit when released.
            {
                let quit = quit.lock().unwrap();
                if *quit {
                    break;
                }
            }

            if let Err(e) = self.writer.write_all(&buffer) {
                if e.kind() == ErrorKind::BrokenPipe {
                    return Ok(());
                }
        
                return Err(e);
            }

        }
        Ok(())
    }
}


pub fn write_loop(outfile: &str, quit: Arc<Mutex<bool>>) -> Result<()> {
    let mut writer = Writer::new(outfile)?;

    writer.write(quit)?;

    Ok(())
}