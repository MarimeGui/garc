extern crate clap;
extern crate garc;

use clap::{App, Arg};
use garc::GARC;
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::BufReader;
use std::path::Path;

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
        )
        .get_matches();

    let input_str = matches.value_of("IN").unwrap();
    let output_str = matches.value_of("OUT").unwrap();
    let input_path = Path::new(input_str);
    let output_path = Path::new(output_str);
    if !input_path.exists() {
        eprintln!("Error: The specified input file does not exist or is unaccessible.");
        return;
    }
    create_dir_all(output_path).unwrap();

    let garc_reader = &mut BufReader::new(File::open(input_path).unwrap());

    let garc = GARC::import(garc_reader).unwrap();

    let nb_files = garc.get_nb_files().unwrap();
    println!("Extracting {} files", nb_files);

    let nb_chars = nb_files.to_string().chars().count();

    for i in 0..nb_files {
        // Need to open with OpenOptions because the create function in File does not allow reading by default
        let file_writer = &mut OpenOptions::new().read(true).write(true).create(true).open(output_path.join(format!("dec_{:01$}.bin", i, nb_chars))).unwrap();
        garc.extract(garc_reader, file_writer, i as usize).unwrap();
    }
}
