mod args;
mod reader;
mod writer;
mod stats;

use args::Args;
use std::io::Result;
use std::sync::{Arc, Mutex};
use std::thread;

const CHUNK_SIZE: usize = 8 * 1024;

fn main() -> Result<()> {
    let args = Args::parse_args();
    let mut total_bytes = 0;

    let quit = Arc::new(Mutex::new(false));
    let (quit1, quit2, quit3) = (quit.clone(), quit.clone(), quit.clone());


    let read_handle = thread::spawn(move || reader::read_loop(&args.infile, quit1));
    let stats_handle = thread::spawn(move || stats::stats(args.silent, quit2));
    let write_handle = thread::spawn(move || writer::write_loop(&args.outfile, quit3));

    let read_io_result= read_handle.join().unwrap();
    let write_io_result = write_handle.join().unwrap();
    let stats_io_result = stats_handle.join().unwrap();

    read_io_result?;
    write_io_result?;
    stats_io_result?;

    Ok(())
}