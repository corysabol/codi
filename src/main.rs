use clap::{Parser, Subcommand};
use std::io::{self, Read};
use urlencoding::encode;

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Args {
    /// Required argument which is a string list of transformations to apply to the input - eg "url
    /// b64 hex"
    encodings: String,
}

fn main() {
    let args = Args::parse();

    let mut input = String::new();
    // Read from stdin
    // We operate on the entire string
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read from stdin");

    let transformations = args.encodings.split_whitespace().collect::<Vec<_>>();

    let mut output = input;
    for transformation in transformations {
        output = match transformation {
            "url" => {
                todo!("URL encoding");
            }
            "b64" => {
                todo!("base64 encoding");
            }
            _ => output, // just ignore unknown encodings
        }
    }
}
