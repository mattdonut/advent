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
    // Read the file
    let contents = fs::read_to_string(config.filename)?;

    // Vector Traits!!!
    let counts = contents
        .split("\n\n") // Split on \n\n (empty line) to seperate the elves
        .map(|s| {
            // map to get the total value for each elf
            s.split("\n")
                .filter_map(|n| n.parse::<i32>().ok()) // filter_map is the best: try to map and throw out anything that doesn't work
                .sum::<i32>() // we ended up with numbers, add them up for each elf seperately
        });
    // The counts iterator will produce all of the totals for the elves
    let max = counts.max().unwrap(); // a bit unsafe, but its Advent! :D
    println!("Max Elf has {:?} Calories", max);
    Ok(())
}
