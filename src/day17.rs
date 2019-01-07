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
                .map(|xx| y.iter().map(move |yy| (xx, *yy)))
                .flat_map(|i| i.into_iter())
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
}
