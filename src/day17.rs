use fnv::FnvHashMap;
use from_pest::FromPest;
use pest::Parser;
use std::str::FromStr;

#[derive(Parser)]
#[grammar = "day17.pest"]
/// Pest Parser for Day17
pub struct Day17Parser;

#[derive(Debug, PartialEq, Eq)]
/// A Value is either an unique i32 value, or a range of i32 values
pub enum Value {
    Unique(i32),
    Range(i32, i32),
}

impl Value {
    /// Gets the numbers represented by the value as a vector of i32
    pub fn as_vec(&self) -> Vec<i32> {
        match self {
            Value::Unique(x) => vec![*x],
            Value::Range(min, max) => (*min..=*max).collect::<Vec<i32>>(),
        }
    }
}

/// Implements the behaviour to parse a Value
impl FromStr for Value {
    type Err = ();

    /// Parses a &str to a `Value`
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input: &str = input.split("=").collect::<Vec<&str>>()[0];
        if input.contains("..") {
            let splitted: Vec<&str> = input.split("..").collect();
            return Ok(Value::Range(
                splitted[0].parse::<i32>().expect("Failed to parse number"),
                splitted[1].parse::<i32>().expect("Failed to parse number"),
            ));
        }
        Ok(Value::Unique(
            input.parse::<i32>().expect("Failed to parse number"),
        ))
    }
}

mod day17_ast {
    use super::Rule;
    use pest::Span;

    /// Transforms a Pest span into a &str
    fn span_into_str(span: Span) -> &str {
        span.as_str()
    }

    /// Extracts the first char of the given &str (useful for the axes)
    fn extract_char(input: &str) -> char {
        input.chars().next().expect("Empty string")
    }

    #[derive(Debug, FromPest)]
    #[pest_ast(rule(Rule::info))]
    pub struct Info {
        pub rule_a: RulePart,
        pub rule_b: RulePart,
    }

    impl Info {
        /// Generates the coordinates given by the ranges of an info
        pub fn get_coordinates(&self) -> Vec<(i32, i32)> {
            let x = match self.rule_a.axis {
                'x' => self.rule_a.value.as_vec(),
                _ => self.rule_b.value.as_vec(),
            };

            let y = match self.rule_a.axis {
                'y' => self.rule_a.value.as_vec(),
                _ => self.rule_b.value.as_vec(),
            };

            x.into_iter()
                .flat_map(|xx| y.iter().map(move |yy| (xx, *yy)))
                .collect()
        }
    }

    #[derive(Debug, FromPest)]
    #[pest_ast(rule(Rule::rule_part))]
    pub struct RulePart {
        #[pest_ast(inner(with(span_into_str), with(extract_char)))]
        pub axis: char,
        #[pest_ast(inner(with(span_into_str), with(str::parse), with(Result::unwrap)))]
        pub value: super::Value,
    }

    #[derive(Debug, FromPest)]
    #[pest_ast(rule(Rule::infos))]
    pub struct Infos {
        infos: Vec<Info>,
    }

    impl Infos {
        pub fn consume(self) -> Vec<Info> {
            self.infos
        }
    }
}

#[aoc_generator(day17)]
fn parse_input(input: &str) -> Vec<day17_ast::Info> {
    let mut parse_tree = Day17Parser::parse(Rule::infos, input).expect("Failed to parse input");
    day17_ast::Infos::from_pest(&mut parse_tree)
        .expect("Failed to parse input")
        .consume()
}

#[derive(Debug, PartialEq, Eq)]
pub enum WaterState {
    Flood,
    Rest,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TileState {
    Water(WaterState),
    Clay,
    Empty,
}

pub struct World {
    tiles: FnvHashMap<(i32, i32), TileState>,
    y_limit: i32,
}

impl World {
    /// Creates a new world with the given infos
    pub fn new(infos: &[day17_ast::Info]) -> Self {
        let mut y_limit = 0;
        let mut tiles: FnvHashMap<(i32, i32), TileState> = infos
            .into_iter()
            .flat_map(|i| i.get_coordinates())
            .map(|c| {
                if y_limit < c.1 {
                    y_limit = c.1;
                }
                (c, TileState::Clay)
            })
            .collect();
        tiles.insert((500, 0), TileState::Water(WaterState::Flood));
        World { tiles, y_limit }
    }

    /// Ticks the world to create new water Floods accordingly.
    /// Returns false if nothing has changed during the tick
    pub fn tick(&mut self) -> bool {
        // Computes the change in state of the World, only the Floods are taken into account
        let state_change: Vec<((i32, i32), TileState)> = self
            .tiles
            .iter()
            .filter_map(|(k, v)| {
                if k.1 > self.y_limit || v != &TileState::Water(WaterState::Flood) {
                    return None;
                }
                let down = (k.0, k.1 + 1);
                match self.get_tile(down) {
                    TileState::Clay | TileState::Water(WaterState::Rest) => {
                        let right = (k.0 + 1, k.1);
                        match self.get_tile(right) {
                            TileState::Water(WaterState::Flood) | TileState::Empty => None,
                            _ => {
                                let left = (k.0 - 1, k.1);
                                match self.get_tile(left) {
                                    TileState::Water(WaterState::Flood) | TileState::Empty => None,
                                    _ => Some((k.clone(), TileState::Water(WaterState::Rest))),
                                }
                            }
                        }
                    }
                    _ => None,
                }
            })
            .collect();

        println!("{} changes", state_change.len());
        println!(
            "putting {} to rest",
            state_change
                .iter()
                .filter(|&v| v.1 == TileState::Water(WaterState::Rest))
                .count()
        );

        let to_add: Vec<((i32, i32), TileState)> = self
            .tiles
            .iter()
            .filter_map(|(k, v)| {
                if k.1 > self.y_limit || v != &TileState::Water(WaterState::Flood) {
                    return None;
                }

                let down = (k.0, k.1 + 1);
                match self.get_tile(down) {
                    TileState::Clay | TileState::Water(WaterState::Rest) => {
                        let left = (k.0 - 1, k.1);
                        let right = (k.0 - 1, k.1);
                        let mut add: Vec<((i32, i32), TileState)> = vec![];
                        if self.get_tile(left) == &TileState::Empty {
                            add.push((left, TileState::Water(WaterState::Flood)));
                        }
                        if self.get_tile(right) == &TileState::Empty {
                            add.push((right, TileState::Water(WaterState::Flood)));
                        }
                        Some(add)
                    }
                    TileState::Empty => Some(vec![((down, TileState::Water(WaterState::Flood)))]),
                    _ => None,
                }
            })
            .flat_map(|i| i.into_iter())
            .collect();

        let count = to_add.len();
        println!("adding: {}", count);
        self.apply_changes(to_add);

        count != 0
    }

    /// Applies the change of state to the tiles
    pub fn apply_changes(&mut self, state_change: Vec<((i32, i32), TileState)>) {
        state_change.into_iter().for_each(|i| {
            let entry = self.tiles.entry(i.0).or_insert(TileState::Empty);
            *entry = i.1;
        });
    }

    /// Gets the tile state at given coords
    pub fn get_tile(&self, coords: (i32, i32)) -> &TileState {
        self.tiles.get(&coords).unwrap_or(&TileState::Empty)
    }

    /// Counts the number of Water tiles -- be it Flood or Rest
    /// - 1 because we originally added the spring of Water as a Flood
    pub fn count_water(&self) -> usize {
        self.tiles
            .values()
            .filter(|&t| {
                t == &TileState::Water(WaterState::Flood)
                    || t == &TileState::Water(WaterState::Rest)
            })
            .count()
            - 1
    }
}

#[aoc(day17, part1)]
/// Solves part one
fn part_one(input: &[day17_ast::Info]) -> usize {
    let mut world = World::new(input);
    (0..).find(|x| {
        println!("step {}", x);
        !world.tick()
    });
    world.count_water()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &str = "x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504";

    #[test]
    fn day17_parse() {
        let infos = parse_input(INPUT);
        assert_eq!(infos.len(), 8);

        // Infos[0] is x=495, y=2..7
        assert_eq!(infos[0].rule_a.axis, 'x');
        assert_eq!(infos[0].rule_a.value, Value::Unique(495));
        assert_eq!(infos[0].rule_b.axis, 'y');
        assert_eq!(infos[0].rule_b.value, Value::Range(2, 7));

        // Infos[1] is y=7, x=495..501
        assert_eq!(infos[1].rule_a.axis, 'y');
        assert_eq!(infos[1].rule_a.value, Value::Unique(7));
        assert_eq!(infos[1].rule_b.axis, 'x');
        assert_eq!(infos[1].rule_b.value, Value::Range(495, 501));

        // .... And so on.
    }

    #[test]
    fn day17_get_coordinates() {
        let info = day17_ast::Info {
            rule_a: day17_ast::RulePart {
                axis: 'x',
                value: Value::Unique(5),
            },
            rule_b: day17_ast::RulePart {
                axis: 'y',
                value: Value::Range(5, 8),
            },
        };
        assert_eq!(info.get_coordinates(), vec![(5, 5), (5, 6), (5, 7), (5, 8)]);
    }

    #[test]
    fn day17_world() {
        let infos = parse_input(INPUT);
        let world = World::new(&infos);
        assert_eq!(
            world
                .tiles
                .values()
                .filter(|&v| v == &TileState::Clay)
                .count(),
            34
        );
        assert_eq!(world.count_water(), 0);
    }

    #[test]
    fn day17_part_one() {
        let infos = parse_input(INPUT);
        assert_eq!(part_one(&infos), 57);
    }
}
