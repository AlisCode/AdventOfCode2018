use fnv::FnvHashMap;
use pest::Parser;
use std::collections::HashMap;
use std::convert::AsRef;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Plants {
    pub state: FnvHashMap<i32, char>,
    pub rules: HashMap<String, char>,
}

impl Plants {
    pub fn next(&mut self) {
        let state_len = self.state.len() as i32;
        let new_state: FnvHashMap<i32, char> = (-1..state_len + 1)
            .map(|i| {
                let pattern: String = (i - 2..=i + 2)
                    .map(|ii| self.state.get(&ii).unwrap_or(&'.'))
                    .collect();
                (i, *self.rules.get(&pattern).unwrap_or(&'.'))
            })
            .collect();
        self.state = new_state;
    }

    pub fn count_plants(&self) -> usize {
        self.state.values().filter(|&c| *c == '#').count()
    }

    pub fn string_repr(&self) -> String {
        let mut vals: Vec<(i32, char)> = self.state.iter().map(|(id, c)| (*id, *c)).collect();
        vals.sort_by(|a, b| a.0.cmp(&b.0));
        vals.into_iter().map(|i| i.1).collect()
    }
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

    let init_state: FnvHashMap<i32, char> = global
        .next()
        .expect("Failed to get init state")
        .as_str()
        .replace("initial state: ", "")
        .trim()
        .chars()
        .enumerate()
        .map(|(id, x)| (id as i32, x))
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
fn part_one(input: &Plants) -> usize {
    let mut plants = input.clone();
    (0..19)
        .map(|_| {
            plants.next();
            plants.count_plants()
        })
        .sum::<usize>()
        + input.count_plants()
}

#[cfg(test)]
pub mod tests {

    use super::*;

    const INPUT: &str = "initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #";

    #[test]
    fn day12_next() {
        let mut plants = gen_plants(INPUT);

        let expected = "#..#.#..##......###...###".to_string();
        let actual: String = plants.string_repr();
        assert_eq!(expected, actual);

        plants.next();

        let expected = ".#...#....#.....#..#..#..#.";
        let actual: String = plants.string_repr();
        assert_eq!(expected, actual);

        plants.next();

        let expected = ".##..##...##....#..#..#..##..";
        let actual: String = plants.string_repr();
        assert_eq!(expected, actual);
    }

    #[test]
    fn day12_part_one() {
        let plants = gen_plants(INPUT);
        assert_eq!(part_one(&plants), 325);
    }
}
