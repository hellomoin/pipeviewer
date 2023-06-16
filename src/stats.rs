pub fn stats(silent: bool, num_read: usize, total_bytes: &mut usize, last: bool) {
    *total_bytes += num_read;

    if !silent {
        eprint!("\r{}", total_bytes);
        if last {
            eprintln!();
        }
    }

    // if let Err(e) = writer.write_all(&buffer[..num_read]) {
    //     if e.kind() == ErrorKind::BrokenPipe {
    //         break;
    //     }
    //     return Err(e);
    //     //std::process::exit(1);
    // }    
}