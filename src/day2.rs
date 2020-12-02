use advent2020::Config;

use std::error::Error;
use std::fs;
use regex::Regex;
use std::string::String;

#[derive(Debug)]
struct RuledPassword {
    min: usize,
    max: usize,
    letter: char,
    password: String
}

impl RuledPassword {
    pub fn new(raw: &str) -> Option<RuledPassword> {
        // Ruled passwords look like: "1-3 a: abcde""
        let re = Regex::new(r"^(\d+)-(\d+) (.): ([^ ]+)$").unwrap(); 
        if let Some(cap) = re.captures(raw) {
            Some(RuledPassword{
                min: cap[1].parse::<usize>().unwrap(),
                max: cap[2].parse::<usize>().unwrap(),
                letter: cap[3].chars().next().unwrap(),
                password: cap[4].to_string(),
            })
        } else {
            None
        }
    }

    pub fn valid(&self) -> bool {
        let count = self.password.chars().filter(|x| *x == self.letter).count();
        count >= self.min && count <= self.max
    }

    pub fn valid2(&self) -> bool {
        let mut got_min = false;
        let mut got_max = false;
        for (i, c) in self.password.char_indices() {
            if c == self.letter {
                if i + 1 == self.min { got_min = true; }
                if i + 1 == self.max { got_max = true; }
            }
        }

        !(got_min && got_max) && (got_min || got_max)
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Running day 2 on {}", config.filename);
    let contents = fs::read_to_string(config.filename)?;
    let contents = contents.lines();
    let valid = contents.map(|x| RuledPassword::new(x)).filter(|x| x.as_ref().unwrap().valid2()).count();
    println!("Found {} valid passwords.", valid);
    let rp = RuledPassword::new("1-3 a: abcdeaaaa");
    if let Some(p) = rp {
    println!("{:?} {:?}", p, p.valid());
    }

    Ok(())
}
