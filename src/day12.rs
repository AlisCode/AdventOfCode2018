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
        let min = self.state.keys().min().expect("Failed to find min");
        let max = self.state.keys().max().expect("Failed to find max");
        let new_state: FnvHashMap<i32, char> = (min - 2..max + 2)
            .filter_map(|i| {
                let pattern: String = (i - 2..=i + 2)
                    .map(|ii| self.state.get(&ii).unwrap_or(&'.'))
                    .collect();
                let c = *self.rules.get(&pattern).unwrap_or(&'.');
                if c == '.' && !self.state.contains_key(&i) {
                    return None;
                }
                Some((i, c))
            })
            .collect();
        self.state = new_state;
    }

    pub fn count_plants(&self) -> i32 {
        self.state
            .iter()
            .filter_map(|(i, c)| if *c == '#' { Some(i) } else { None })
            .sum()
    }

    pub fn number_plants(&self) -> usize {
        self.state.values().filter(|&c| *c == '#').count()
    }

    #[cfg(test)]
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
fn part_one(input: &Plants) -> i32 {
    let mut plants = input.clone();
    (0..20).for_each(|_| plants.next());
    plants.count_plants()
}

#[aoc(day12, part2)]
fn part_two(input: &Plants) -> i64 {
    let mut plants = input.clone();
    // Plants will eventually all become the same pattern repeating over and over
    let mut times_same = 0;
    let mut plant_nb = 0;
    // So let's find the index where the number of plants stay the same ..
    let convergence_id = (0..)
        .find(|_| {
            plants.next();
            let nb = plants.number_plants();
            if nb == plant_nb {
                times_same += 1;
                // ... about a hundred times
                return times_same >= 100;
            } else {
                plant_nb = nb;
            }
            false
        })
        .expect("Failed to find convergence");

    // Compute the score (part one) of the current scheme
    // All plants move by 1 each iteration, so the final score will
    // eventually be: score + number of plants * offset
    let offset: i64 = 50_000_000_000 - convergence_id - 1;
    plants.count_plants() as i64 + plants.number_plants() as i64 * offset
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

        let expected: Vec<&str> = vec![
            "#..#.#..##......###...###",
            "#...#....#.....#..#..#..#",
            "##..##...##....#..#..#..##",
            "#.#...#..#.#....#..#..#...#",
            ".#.#..#...#.#...#..#..##..##",
            "..#...##...#.#..#..#...#...#",
            "..##.#.#....#...#..##..##..##",
            ".#..###.#...##..#...#...#...#",
            ".#....##.#.#.#..##..##..##..##",
            ".##..#..#####....#...#...#...#",
            "#.#..#...#.##....##..##..##..##",
            ".#...##...#.#...#.#...#...#...#",
            ".##.#.#....#.#...#.#..##..##..##",
            "#..###.#....#.#...#....#...#...#",
            "#....##.#....#.#..##...##..##..##",
            "##..#..#.#....#....#..#.#...#...#",
            "#.#..#...#.#...##...#...#.#..##..##",
            ".#...##...#.#.#.#...##...#....#...#",
            ".##.#.#....#####.#.#.#...##...##..##",
            "#..###.#..#.#.#######.#.#.#..#.#...#",
            "#....##....#####...#######....#.#..##",
        ];

        (0..expected.len()).for_each(|i| {
            assert_eq!(&plants.string_repr(), expected[i]);
            plants.next();
        });
    }

    #[test]
    fn day12_part_one() {
        let plants = gen_plants(INPUT);
        assert_eq!(part_one(&plants), 325);
    }
}
