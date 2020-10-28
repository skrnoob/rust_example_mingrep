use minigrep::{Config,run};

use std::env;
use std::process;


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("{}",err);
        process::exit(1);
    });

    if let Err(err) = run(config) {
        eprintln!("{}",err);
        process::exit(1);
    };
}


