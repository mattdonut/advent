use day2_1::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem getting args!: {}", err);
        process::exit(1);
    });
    if let Err(e) = day2_1::run(config) {
        println!("Application Error: {}", e);
        process::exit(1);
    }
}
