use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write, BufWriter};
use std::path::Path;

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
        .author("Myron Lioz <liozmyron@gmail.com>")
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
    let mut buf_read = open(&config.in_file).map_err(|e| format!("{}: {}", &config.in_file, e))?;
    
    let buf_write = match config.out_file {
        None => Box::new(BufWriter::new(io::stdout())),
        Some(out_filename) => Box::new(BufWriter::new(File::create(Path::new(&out_filename)?))),
    };

    let mut prev_line = String::new();

    let bytes_read = buf_read.read_line(&mut prev_line)?;
    if bytes_read == 0 {
        return Ok(());
    }

    let mut unique_count = 1;
    loop {
        let mut current_line = String::new();

        let bytes_read = buf_read.read_line(&mut current_line)?;

        if bytes_read != 0 && prev_line.eq(&current_line) {
            if config.count {
                unique_count += 1;
            }

            continue;
        }

        match out_file {
            None => 
        }
        write!(
            "{}{}",
            if config.count {
                format!("{:>4} ", unique_count)
            } else {
                "".to_string()
            },
            &prev_line
        );

        if bytes_read == 0 {
            break;
        }

        unique_count = 1;

        prev_line = current_line;
    }

    Ok(())
}
