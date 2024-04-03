use base64::prelude::*;
use clap::{Parser, Subcommand};
use sha1::Sha1;
use sha2::{Digest, Sha256};
use std::io::{self, Read};
use urlencoding::encode;

#[derive(Parser)]
#[command(
    author = "Cory Sabol",
    version = "0.1.0",
    about = "en(coder) is a simple tool to perform a sequence of encodings and hashings on input from STDIN and print the result to STDOUT",
    after_help = "Examples:
    echo \"hey\" | en \"bin\" -> 01101000 01100101 01111001 01010
    echo \"<script>alert(1)</script>\" | en \"html url\" -> %26lt%3Bscript%26gt%3Balert%281%29%26lt%3B%26%23x2F%3Bscript%26gt%3B%0A
    echo \":)\" | en \"b64 md5 sha1 sha256 bin\" -> 01100110 01100010 01100011 01100101...
    "
)]
struct Args {
    /// Required argument which is a string list of transformations to apply to the input.
    /// Transformations can be chained in sequence.
    /// E.g. "html url b64 hex"
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
