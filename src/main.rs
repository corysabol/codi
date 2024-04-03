use base64::prelude::*;
use clap::{Parser, Subcommand};
use sha1::Sha1;
use sha2::{Digest, Sha256};
use std::io::{self, Read};
use urlencoding::encode;

#[derive(Parser)]
#[command(version="0.1.0", about="en is a simple tool for performing various encoding operations on STDIN and printing the result to STDOUT", long_about=None)]
struct Args {
    /// Required argument which is a string list of transformations to apply to the input - eg "url
    /// b64 hex"
    encodings: String,
}

fn into_bin(input: &[u8]) -> String {
    let mut output = "".to_string();
    for c in input {
        output += &format!("0{:b} ", c);
    }
    output
}

fn into_sha1(input: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(input);
    let result = hasher.finalize();
    format!("{:x}", result)
}

fn into_sha256(input: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    format!("{:x}", result)
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
            "url" => urlencoding::encode(&output).to_string(),
            "b64" => data_encoding::BASE64.encode(&output.as_bytes()),
            "b32" => data_encoding::BASE32.encode(&output.as_bytes()),
            "html" => html_escape::encode_safe(&output).to_string(),
            "bin" => into_bin(&output.as_bytes()),
            "hexlo" => data_encoding::HEXLOWER.encode(&output.as_bytes()),
            "hexup" => data_encoding::HEXUPPER.encode(&output.as_bytes()),
            "md5" => format!("{:x}", md5::compute(&output)),
            "sha1" => into_sha1(&output.as_bytes()),
            "sha256" => into_sha256(&output.as_bytes()),
            _ => output, // just ignore unknown encodings
        }
    }

    println!("{output}");
}
