use std::error::Error;
use clap::{App,Arg};
use std::io::{self, BufRead, BufReader};
use std::fs::File;

type MyResult<T> = Result<T, Box<dyn Error>>;
#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> MyResult<()> {   
    for filename in config.files {
        let mut idx = 0;
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(bufreader) => {
                for line in bufreader.lines() {
                    match line {
                        Err(err) => eprintln!("{}", err),
                        Ok(linestr) => {
                            idx += 1;
                            if (!config.number_lines && !config.number_nonblank_lines) || (config.number_nonblank_lines && linestr.is_empty()) {
                                idx -= 1;
                                print_formatted_line("", &linestr, false);
                            } else {
                                let lineidx = "     ".to_string() + &idx.to_string() + "\t";
                                print_formatted_line(&lineidx, &linestr, true);
                            }
                        },
                    }
                }
            },
        }
    }
    Ok(())
}

fn print_formatted_line(lineidx: &str, linestr: &str, print_line_number: bool) {
    match print_line_number {
        true => {
            println!("{}{}",lineidx,linestr);
        },
        false => {
            println!("{}", linestr);
        },
    }
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
    .about("Rust Cat")
    .author("Harish R <dev@rhrish.com>")
    .version("0.1.0")
    .arg(
        Arg::with_name("number-nonblank")
        .short("b")
        .long("number-nonblank")
        .help("number nonempty output lines, overrides -n")
        .required(false)
    )
    .arg(
        Arg::with_name("number")
        .short("n")
        .long("number")
        .conflicts_with("number-nonblank")
        .help("number all output lines")
        .required(false)
    )
    .arg(
        Arg::with_name("FILE")
        .help("Input file(s)")
        .default_value("-")
        .takes_value(true)
        .multiple(true)
    )
    .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("FILE").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number-nonblank"),
    })
}