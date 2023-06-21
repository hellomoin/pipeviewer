use std::{io::{self, ErrorKind,BufWriter, Write, Result}};
use std::fs::File;
use std::sync::mpsc::Receiver;

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

    pub fn write(&mut self, write_rx: Receiver<Vec<u8>>) -> Result<()> {
        loop {
            let buffer = write_rx.recv().unwrap();
            
            if buffer.is_empty() {
                break;
            }

            if let Err(e) = self.writer.write_all(&buffer) {
                if e.kind() == ErrorKind::BrokenPipe {
                    return Ok(());
                }
        
                return Err(e);
            }

            break;
        }
        Ok(())
    }
}


pub fn write_loop(outfile: &str, write_rx: Receiver<Vec<u8>>) -> Result<()> {
    let mut writer = Writer::new(outfile)?;

    writer.write(write_rx)?;

    Ok(())
}