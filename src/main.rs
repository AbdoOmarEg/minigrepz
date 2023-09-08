use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<_> = env::args().collect::<Vec<String>>();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem passing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
