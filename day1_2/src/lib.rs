use std::error::Error;
use std::fs;

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough args");
        }

        let filename = args[1].clone();
        Ok(Config { filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    // let nums: Vec<&str> = contents.split(",").collect();
    let mut counts = contents
        .split("\n\n")
        .map(|s| {
            s.split("\n")
                .filter_map(|n| n.parse::<i32>().ok())
                .sum::<i32>()
        })
        .collect::<Vec<i32>>();
    counts.sort();
    let tops: i32 = counts.iter().rev().take(3).sum();
    println!("Top Elves {:?}", tops);
    Ok(())
}
