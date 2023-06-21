mod args;
mod reader;
mod writer;
mod stats;

use args::Args;
use std::io::Result;
use std::sync::mpsc;
use std::thread;

const CHUNK_SIZE: usize = 8 * 1024;

fn main() -> Result<()> {
    let args = Args::parse_args();

    let (stats_tx, stats_rx) = mpsc::channel();
    let (write_tx, write_rx) = mpsc::channel();

    let read_handle = thread::spawn(move || reader::read_loop(&args.infile, stats_tx));
    let stats_handle = thread::spawn(move || stats::stats(args.silent, stats_rx, write_tx));
    let write_handle = thread::spawn(move || writer::write_loop(&args.outfile, write_rx));

    let read_io_result= read_handle.join().unwrap();
    let write_io_result = write_handle.join().unwrap();
    let stats_io_result = stats_handle.join().unwrap();

    read_io_result?;
    write_io_result?;
    stats_io_result?;

    Ok(())
}