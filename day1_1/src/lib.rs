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
    let counts = contents
        .split("\n\n")
        .map(|s| {
            s.split("\n")
                .filter_map(|n| n.parse::<i32>().ok())
                .sum::<i32>()
        })
        .collect::<Vec<i32>>();
    let max = counts.iter().max().unwrap();
    let position = counts.iter().position(|val| val == max);
    println!("Max Elf # {:?} has {:?} Calories", position, max);
    Ok(())
}
