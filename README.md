# en

## 🌟 Overview
`en` (encoder) is a versatile CLI tool designed to perform a series of encodings and hashings on data received from STDIN, outputting the transformed data to STDOUT. It supports a range of transformations, allowing users to chain multiple encoding and hashing operations in sequence for flexible and powerful data manipulation.

## 🚀 Features
- **Chainable Transformations:** Apply a sequence of different encodings and hashings to your input data seamlessly.
- **Supports Multiple Formats:** Includes support for a variety of encoding and hashing schemes, including binary, hexadecimal, Base64, URL encoding, HTML entities, MD5, SHA-1, SHA-256, and more.
- **Simple CLI Usage:** Easy-to-use command-line interface that works with data from STDIN, making it suitable for scripting and automation.

## ⬇️ Installation
Currently, `en` is distributed as a binary executable. To install `en`, download the latest [release](/releases) for your operating system and architecture, and place it in a directory included in your system's PATH.

## 📖 Usage
```plaintext
en(coder) is a simple tool to perform a sequence of encodings and hashings on input from STDIN and print the result to STDOUT

Usage: en <ENCODINGS>

Arguments:
  <ENCODINGS>  Required argument which is a string list of transformations to apply to the input. Transformations can be chained in sequence. E.g. "html url b64 hex"

Options:
  -h, --help     Print help
  -V, --version  Print version

Examples:
    echo "hey" | en "bin" -> 01101000 01100101 01111001 01010
    echo "<script>alert(1)</script>" | en "html url" -> %26lt%3Bscript%26gt%3Balert%281%29%26lt%3B%26%23x2F%3Bscript%26gt%3B%0A
    echo ":)" | en "b64 md5 sha1 sha256 bin" -> 01100110 01100010 01100011 01100101...
```

## 🛠 Supported Transformations
Transformations can be combined by specifying them in sequence. The following are examples of supported transformations:
- `bin`: Convert to binary.
- `hexlo`: Convert to LOWERCASE hex.
- `hexup`: Convert to UPPERCASE hex.
- `b64`: Encode to Base64.
- `b32`: Encode to Base32.
- `url`: Perform URL encoding.
- `html`: Convert characters to HTML entities.
- `md5`: Compute MD5 hash.
- `sha1`: Compute SHA-1 hash.
- `sha256`: Compute SHA-256 hash.
