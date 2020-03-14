use minigrep::{run, Config};
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Err: {}", err);
        process::exit(1)
    });
    println!("find {} in {}", config.query, config.filename);
    println!("{}", "=".repeat(32));

    run(&config).unwrap();
}
