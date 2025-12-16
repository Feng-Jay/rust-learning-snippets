use std::env;
use std::process;
use minigrep::{Config, run};

fn main() {
    // let args: Vec<String> = env::args().collect();
    // dbg!(&args);
    // let config = parse_args(&args);
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    // println!("Searching for {}", config.query());
    // println!("In the file {}", config.file_path());

    if let Err(e) = run(config){
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}


