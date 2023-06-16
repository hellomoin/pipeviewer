use clap::{Parser, command};
use std::env;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]

pub struct Args {
    #[arg(short, long, default_value_t = String::from(""))]
    pub infile: String,
    #[arg(short, long, default_value_t = String::from(""))]
    pub outfile: String,
    /// Number of times to greet
    #[arg(short, long, default_value_t = false)]
    pub silent: bool,
}

impl Args {
    /// .parse file
    pub fn parse_args() -> Self {
        let args = Args::parse();

        let silent = if args.silent {
            true
        } else {
            !env::var("PV_SILENT").unwrap_or_default().is_empty()
        };

        let infile = args.infile;
        let outfile = args.outfile;

        Self {
            infile,
            outfile,
            silent
        }
    }
}
