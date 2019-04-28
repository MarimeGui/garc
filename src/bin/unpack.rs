extern crate clap;
extern crate garc;

use clap::{App, Arg};
use std::fs::{create_dir_all, File};
use std::path::Path;
use std::io::BufReader;
use garc::GARC;

fn main() {
    let matches = App::new("GARC Unpacker")
        .version("0.1")
        .author("MarimeGui <lepro.guillaume@gmail.com>")
        .about("Extracts GARC files found in 3DS Titles like Pokémon games")
        .arg(
            Arg::with_name("IN")
                .help("Input GARC file")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("OUT")
                .help("Output Folder")
                .required(true)
                .index(2),
        ).get_matches();

    let input_str = matches.value_of("IN").unwrap();
    let output_str = matches.value_of("OUT").unwrap();
    let input_path = Path::new(input_str);
    let output_path = Path::new(output_str);
    if !input_path.exists() {
        eprintln!("Error: The specified input file does not exist or is unaccessible.");
        return;
    }
    create_dir_all(output_path).unwrap();

    let garc = GARC::import(&mut BufReader::new(File::open(input_path).unwrap())).unwrap();

    println!("Extracting {} files", garc.fatb.nb_entries);
}
