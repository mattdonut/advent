use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

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

pub fn round_outcome(me: &str, them: &str) -> i32 {
    return match them {
        "A" => match me {
            "X" => 3,
            "Y" => 4,
            "Z" => 8,
            _ => 0,
        },
        "B" => match me {
            "X" => 1,
            "Y" => 5,
            "Z" => 9,
            _ => 0,
        },
        "C" => match me {
            "X" => 2,
            "Y" => 6,
            "Z" => 7,
            _ => 0,
        },
        _ => 0,
    };
}

pub fn score_round(line: Result<String, std::io::Error>) -> i32 {
    if let Ok(plan) = line {
        let parts = plan.split(' ').collect::<Vec<&str>>();
        return round_outcome(parts[1], parts[0]);
    }
    return 0;
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    if let Ok(lines) = read_lines(config.filename) {
        let total = lines.map(score_round).sum::<i32>();
        println!("Total score for plan is: {:?}", total);
    }

    Ok(())
}
