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
    let mut counts = contents
        .split("\n\n")
        .map(|s| {
            s.split("\n")
                .filter_map(|n| n.parse::<i32>().ok())
                .sum::<i32>()
        })
        .collect::<Vec<i32>>(); // we need to collect the iterator so we can sort it
                                // Sort the counts vector
    counts.sort();

    // reverse the counts and take the first 3 (these will be the biggest), then add them up
    let tops: i32 = counts.iter().rev().take(3).sum();
    println!("Top Elves {:?}", tops);
    Ok(())
}
