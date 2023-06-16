mod args;
mod read;
mod write;
mod stats;

use args::Args;
use read::Reader;
use write::Writer;

use std::io::Result;

const CHUNK_SIZE: usize = 8 * 1024;

fn main() -> Result<()> {
    let args = Args::parse_args();
    let mut total_bytes = 0;

    let mut reader = Reader::new(&args.infile)?;
    let mut writer = Writer::new(&args.outfile)?;

    loop {
        let buffer = reader.read()?;

        stats::stats(args.silent, buffer.len(), &mut total_bytes, false);

        if !writer.write(&buffer)? {
            break;
        }
    }
    stats::stats(args.silent, 0, &mut total_bytes, true);

    Ok(())
}