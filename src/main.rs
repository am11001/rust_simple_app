use std::env;
use std::process;

use rust_simple_app::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = rust_simple_app::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
