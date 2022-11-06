use std::error::Error;
use std::io::{self, BufRead, BufReader};
use std::fs::File;

use clap::{App, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    in_file: String,
    out_file: Option<String>,
    count: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("uniqr")
        .about("Rust uniq")
        .author("Myron Lioz <liozmyron@gmail.com")
        .version("0.1.0")
        .arg(
            Arg::with_name("in_file")
                .value_name("IN_FILE")
                .takes_value(true)
                .help("Input file")
                .default_value("-"),
        )
        .arg(
            Arg::with_name("out_file")
                .value_name("OUT_FILE")
                .help("Output file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("count")
                .short("c")
                .long("long")
                .takes_value(false)
                .help("prefix lines by the number of occurrences"),
        )
        .get_matches();

    Ok(Config {
        in_file: matches.value_of("in_file").unwrap().to_string(),
        out_file: matches.value_of("out_file").map(String::from),
        count: matches.is_present("count"),
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    let mut buf_read = open(&config.in_file)
        .map_err(|e| format!("{}: {}", &config.in_file, e))?;

    let mut buffer = String::new();

    while let Ok(bytes_read) = buf_read.read_line(&mut buffer) {
        if bytes_read == 0 {
            break;
        }

        print!("{}", buffer);
        buffer.clear();
    }

    Ok(())
}
