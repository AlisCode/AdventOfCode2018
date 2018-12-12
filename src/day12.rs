use pest::Parser;
use std::collections::HashMap;
use std::convert::AsRef;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Plants {
    pub state: Vec<char>,
    pub rules: HashMap<String, char>,
}

impl AsRef<Plants> for Plants {
    fn as_ref(&self) -> &Plants {
        self
    }
}

mod day12 {
    #[derive(Parser)]
    #[grammar = "day12.pest"]
    pub struct PlantParser;
}

pub struct Rule {
    pattern: String,
    output: char,
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parts: Vec<String> = input.split("=>").map(|s| s.to_string()).collect();
        let pattern: String = parts[0].trim().to_owned();
        let output: char = parts[1]
            .trim()
            .chars()
            .next()
            .expect("Failed to find output");
        Ok(Rule { pattern, output })
    }
}

#[aoc_generator(day12)]
fn gen_plants(input: &str) -> Plants {
    let mut global = day12::PlantParser::parse(day12::Rule::global, input)
        .expect("Failed to parse")
        .next()
        .unwrap()
        .into_inner();

    let init_state: Vec<char> = global
        .next()
        .expect("Failed to get init state")
        .as_str()
        .replace("initial state: ", "")
        .trim()
        .chars()
        .collect();
    let rules: HashMap<String, char> = global
        .map(|s| {
            let rule = s.as_str().parse::<Rule>().expect("Failed to parse rule");
            (rule.pattern, rule.output)
        })
        .collect();

    Plants {
        state: init_state,
        rules,
    }
}

#[aoc(day12, part1)]
fn part_one(plants: &Plants) -> usize {
    println!("{:?}", plants);
    0
}
