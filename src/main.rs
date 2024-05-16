extern crate clap;
use clap::{App, Arg};

fn main() {
    // println!("Hello, world!");
    let matches = App::new("MyApp")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("FILE")
                .help("Input file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("Output file")
                .takes_value(true),
        )
        .get_matches();
}
