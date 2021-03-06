use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::{App, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Ejaz Ahmed")
        .about("Rust Cat")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input File(s)")
                .required(true)
                .min_values(1)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number_lines")
                .help("Number lines")
                .takes_value(false)
                .short("n")
                .long("number")
                .conflicts_with("number_nonblank_lines"),
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
                .help("Number non blank lines")
                .takes_value(false)
                .short("b")
                .long("number-nonblank"),
        )
        .get_matches();

    let config = Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number_lines"),
        number_nonblank_lines: matches.is_present("number_nonblank_lines"),
    };

    Ok(config)
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                let mut last_num = 0;
                for (i, line) in file.lines().enumerate() {
                    let line = line?;

                    if config.number_lines {
                        println!("{:>6}\t{}", i + 1, line);
                    } else if config.number_nonblank_lines {
                        if !line.is_empty() {
                            last_num += 1;
                            println!("{:>6}\t{}", last_num, line);
                        } else {
                            println!();
                        }
                    } else {
                        println!("{}", line)
                    }
                }
            }
        }
    }

    Ok(())
}

pub fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
