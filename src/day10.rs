use pest::Parser;
use std::ops::AddAssign;
use std::str::FromStr;

#[derive(PartialEq, Eq, Debug)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

#[derive(Parser)]
#[grammar = "day10.pest"]
pub struct StarParser;

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Vec2) {
        self.x += other.x;
        self.y += other.y;
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Star {
    pos: Vec2,
    velocity: Vec2,
}

impl Star {
    pub fn tick(&mut self) {
        self.pos += &self.velocity;
    }
}

impl FromStr for Star {
    type Err = pest::error::Error<Rule>;

    /// Parses star from &str. Example: position=<-3,  6> velocity=< 2, -1>
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut star_parsed = StarParser::parse(Rule::star, input)?;
        let mut star_parsed = star_parsed.next().expect("No star").into_inner();
        let mut pos = star_parsed.next().expect("No position").into_inner();
        let mut vel = star_parsed.next().expect("No velocity").into_inner();

        // Parse pos
        let pos_x: i32 = pos
            .next()
            .expect("No px")
            .as_str()
            .trim()
            .parse()
            .expect("Failed to parse pos_x");
        let pos_y: i32 = pos
            .next()
            .expect("No py")
            .as_str()
            .trim()
            .parse()
            .expect("Failed to parse pos_y");

        // Parse vel
        let vel_x: i32 = vel
            .next()
            .expect("No vx")
            .as_str()
            .parse()
            .expect("Failed to parse vel_x");
        let vel_y: i32 = vel
            .next()
            .expect("No vy")
            .as_str()
            .parse()
            .expect("Failed to parse vel_y");

        Ok(Star {
            pos: Vec2 { x: pos_x, y: pos_y },
            velocity: Vec2 { x: vel_x, y: vel_y },
        })
    }
}

#[aoc_generator(day10)]
pub fn gen_stars(input: &str) -> Vec<Star> {
    input
        .lines()
        .map(|l| l.parse().expect("Failed to parse star"))
        .collect()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn day10_parse_str() {
        assert_eq!(
            "position=< 7,  6> velocity=<-1, -1>"
                .parse::<Star>()
                .expect("Failed to parse"),
            Star {
                pos: Vec2 { x: 7, y: 6 },
                velocity: Vec2 { x: -1, y: -1 }
            }
        );
    }
}
