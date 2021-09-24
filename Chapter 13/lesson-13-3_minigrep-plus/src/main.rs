use minigrep::Config;
use std::env;   // collect
use std::process;

fn main () {
    let cfg = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing argumens: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(cfg) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
