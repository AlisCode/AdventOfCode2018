use fnv::FnvHashMap;
use std::collections::VecDeque;
use std::convert::AsRef;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(PartialEq, Eq, Debug)]
pub struct Rules {
    players: u32,
    highest_marble: u32,
}

impl FromStr for Rules {
    type Err = ParseIntError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = input.trim().split(" ").collect();

        Ok(Rules {
            players: parts[0].parse()?,
            highest_marble: parts[6].parse()?,
        })
    }
}

impl AsRef<Rules> for Rules {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// Generates the rules from the given input
#[aoc_generator(day9)]
fn gen_rules(input: &str) -> Rules {
    input.parse().expect("Failed to parse rules ")
}

/// Solves part one
#[aoc(day9, part1)]
fn part_one(rules: &Rules) -> u32 {
    // Using a Deque with the "current marble" being the front item
    let mut marbles: VecDeque<u32> = VecDeque::with_capacity(rules.highest_marble as usize);
    marbles.push_back(0);
    // Implementable in an FnvHashMap or in a Vec.
    let mut scores: FnvHashMap<u32, u32> = FnvHashMap::default();

    // Iterating over each marble
    (1..=rules.highest_marble).for_each(|x| match x {
        i if i % 23 == 0 => {
            // Computes the ID of the elf player
            let id_elf = i % rules.players;
            let entry = scores.entry(id_elf).or_insert(0);
            // Modifying the entry accordingly
            *entry += i;
            (0..7).for_each(|_| {
                let save = marbles.pop_back().expect("Failed to dequeue");
                marbles.push_front(save);
            });
            *entry += marbles
                .pop_front()
                .expect("Failed to get first circle value");
        }
        i => {
            // Cycle two times
            (0..2).for_each(|_| {
                let saved = marbles.pop_front().unwrap();
                marbles.push_back(saved);
            });
            // push_front so the front element is the "current marble"
            marbles.push_front(i);
        }
    });
    *scores.values().max().expect("Failed to find max score")
}

/// Solves part two
#[aoc(day9, part2)]
fn part_two(rules: &Rules) -> u32 {
    // Well ... Just recompute, changing the rules ...
    let new_rules = Rules {
        players: rules.players,
        highest_marble: rules.highest_marble * 100,
    };
    part_one(&new_rules)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const TEST_EXAMPLE: &str = "9 players; last marble is worth 25 points";
    const TEST_ONE: &str = "10 players; last marble is worth 1618 points";
    const TEST_TWO: &str = "13 players; last marble is worth 7999 points";
    const TEST_THREE: &str = "17 players; last marble is worth 1104 points";
    const TEST_FOUR: &str = "21 players; last marble is worth 6111 points";
    const TEST_FIVE: &str = "30 players; last marble is worth 5807 points";

    #[test]
    fn day9_parse_rules() {
        assert_eq!(
            TEST_ONE.parse::<Rules>().expect("Failed to parse"),
            Rules {
                players: 10,
                highest_marble: 1618
            }
        );

        assert_eq!(
            TEST_TWO.parse::<Rules>().expect("Failed to parse"),
            Rules {
                players: 13,
                highest_marble: 7999
            }
        );

        assert_eq!(
            TEST_THREE.parse::<Rules>().expect("Failed to parse"),
            Rules {
                players: 17,
                highest_marble: 1104
            }
        );

        assert_eq!(
            TEST_FOUR.parse::<Rules>().expect("Failed to parse"),
            Rules {
                players: 21,
                highest_marble: 6111
            }
        );

        assert_eq!(
            TEST_FIVE.parse::<Rules>().expect("Failed to parse"),
            Rules {
                players: 30,
                highest_marble: 5807
            }
        );
    }

    #[test]
    fn day9_part_one() {
        let rules = TEST_EXAMPLE
            .parse::<Rules>()
            .expect("Failed to parse Rules");
        assert_eq!(part_one(&rules), 32);
        assert_eq!(
            part_one(
                &"10 players; last marble is worth 1618 points"
                    .parse::<Rules>()
                    .unwrap()
            ),
            8317
        );
        assert_eq!(
            part_one(
                &"13 players; last marble is worth 7999 points"
                    .parse::<Rules>()
                    .unwrap()
            ),
            146373
        );
        assert_eq!(
            part_one(
                &"17 players; last marble is worth 1104 points"
                    .parse::<Rules>()
                    .unwrap()
            ),
            2764
        );
        assert_eq!(
            part_one(
                &"21 players; last marble is worth 6111 points"
                    .parse::<Rules>()
                    .unwrap()
            ),
            54718
        );
        assert_eq!(
            part_one(
                &"30 players; last marble is worth 5807 points"
                    .parse::<Rules>()
                    .unwrap()
            ),
            37305
        );
    }
}
