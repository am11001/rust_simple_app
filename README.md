# rust_simple_app

This is a simple grep like application written on Rust 

## Installiation 

        cd rust_simple_app
        cargo build


## Usage 

### Case Sensitive Search
        cargo run -- test_text test_file.txt

### Case Insensitive Search
        IGNORE_CASE=1 cargo run -- test_text test_file.txt