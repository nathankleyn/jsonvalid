extern crate clap;
extern crate encoding_rs;
extern crate serde_json;

use clap::{App, Arg};
use encoding_rs::UTF_8;
use serde_json::{Value, Error};

use std::fs::File;
use std::io::Read;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let args = App::new("JSON Valid")
        .version(VERSION)
        .author("Nathan Kleyn <nathan@nathankleyn.com>")
        .about("Checks whether JSON is valid and well-formed.")
        .arg(Arg::from_usage("[FILE]... 'files to validate; if none are given, it will read from stdin instead'"))
        .get_matches();

    let has_errs = match args.values_of("FILE") {
        // At least one path was provided.
        Some(paths) => {
            paths.fold(false, |acc, path| {
                let mut file = File::open(path).expect(&format!("{}: No such file or directory", path));
                let json = string_from_reader(&mut file).expect(&format!("{}: Could not open for reading", path));
                match find_invalid_json(&json) {
                    None => acc,
                    Some(err) => {
                        eprintln!("Error in file {}: {}", path, err);
                        true
                    }
                }
            })
        },
        // No paths were provided at all - read from stdin.
        None => {
            let mut stdin = std::io::stdin();
            let json = string_from_reader(&mut stdin).expect("Failed to read from stdin");
            match find_invalid_json(&json) {
                None => false,
                Some(err) => {
                    eprintln!("Error in JSON: {}", err);
                    true
                }
            }
        }
    };

    ::std::process::exit(match has_errs {
        true => 1,
        false => 0
    })
}

/// Safely decode from a Read trait to a String by correctly handling potential
/// UTF-8 BOMs, etc. To do this, we go via the encoding_rs rather than reading
/// directly to a Rust string which leaves the BOM in place, causing Serde to
/// fail when it sees a codepoint it knows isn't valid JSON.
fn string_from_reader(reader: &mut Read) -> Result<String, std::io::Error> {
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    let (res, _) = UTF_8.decode_with_bom_removal(&buffer);
    Ok(res.to_string())
}

fn find_invalid_json(json: &str) -> Option<Error> {
    serde_json::from_str::<Value>(json).err()
}


// Looking for tests? You can find them in the /tests folder!
