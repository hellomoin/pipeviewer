use std::{io::{self, ErrorKind,BufWriter, Write, Result}};
use std::fs::File;

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

        pub fn write(&mut self, buffer: &[u8]) -> Result<bool> {
            if buffer.len() == 0 {
                return Ok(false);
            }

            if let Err(e) = self.writer.write_all(&buffer) {
                if e.kind() == ErrorKind::BrokenPipe {
                    return Ok(false);
                }
        
                return Err(e);
            }
        
            Ok(true)
        }
}