use pest::Parser;
use std::str::FromStr;

#[derive(PartialEq, Eq, Debug)]
pub struct Vec2 {
    pub x: i64,
    pub y: i64,
}

#[derive(Parser)]
#[grammar = "day10.pest"]
pub struct StarParser;

#[derive(PartialEq, Eq, Debug)]
pub struct Star {
    pos: Vec2,
    velocity: Vec2,
}

impl Star {
    pub fn pos_at_t(&self, t: i64) -> Vec2 {
        Vec2 {
            x: self.pos.x + self.velocity.x * t,
            y: self.pos.y + self.velocity.y * t,
        }
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
        let pos_x: i64 = pos
            .next()
            .expect("No px")
            .as_str()
            .trim()
            .parse()
            .expect("Failed to parse pos_x");
        let pos_y: i64 = pos
            .next()
            .expect("No py")
            .as_str()
            .trim()
            .parse()
            .expect("Failed to parse pos_y");

        // Parse vel
        let vel_x: i64 = vel
            .next()
            .expect("No vx")
            .as_str()
            .trim()
            .parse()
            .expect("Failed to parse vel_x");
        let vel_y: i64 = vel
            .next()
            .expect("No vy")
            .as_str()
            .trim()
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
        .map(|l| l.trim().parse().expect("Failed to parse star"))
        .collect()
}

#[aoc(day10, part1)]
pub fn part_one(input: &[Star]) -> String {
    let i = (0..=15000)
        .map(|i| (compute_bbox_size_at_t(input, i), i))
        .min_by_key(|i| i.0)
        .expect("Failed to find min");
    str_representation(input, i.1)
}

#[aoc(day10, part2)]
pub fn part_two(input: &[Star]) -> i64 {
    let i = (0..=15000)
        .map(|i| (compute_bbox_size_at_t(input, i), i))
        .min_by_key(|i| i.0)
        .expect("Failed to find min");
    i.1
}

fn compute_bbox_size_at_t(stars: &[Star], t: i64) -> i64 {
    let bbox = compute_bbox_at_t(stars, t);
    let val = (bbox.2 - bbox.0).abs() * (bbox.3 - bbox.1).abs();
    val
}

fn compute_bbox_at_t(stars: &[Star], t: i64) -> (i64, i64, i64, i64) {
    let first = &stars[0].pos_at_t(t);
    stars
        .iter()
        .map(|s| s.pos_at_t(t))
        .fold((first.x, first.y, first.x, first.y), |acc, i| {
            (
                acc.0.min(i.x),
                acc.1.min(i.y),
                acc.2.max(i.x),
                acc.3.max(i.y),
            )
        })
}

fn str_representation(input: &[Star], t: i64) -> String {
    let bbox = compute_bbox_at_t(input, t);
    let coords: Vec<Vec2> = input.into_iter().map(|s| s.pos_at_t(t)).collect();
    let mut repr: String = "\n".into();
    println!("{:?}", bbox);
    for y in bbox.1..=bbox.3 {
        for x in bbox.0..=bbox.2 {
            repr.push(match coords.iter().any(|p| p.x == x && p.y == y) {
                true => '#',
                false => '.',
            });
        }
        repr.push('\n');
    }
    repr
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &str = "position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>";

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

        assert_eq!(
            "position=< 7,  6> velocity=<1, 1>"
                .parse::<Star>()
                .expect("Failed to parse"),
            Star {
                pos: Vec2 { x: 7, y: 6 },
                velocity: Vec2 { x: 1, y: 1 }
            }
        );

        assert_eq!(
            "position=<-39906, -49878> velocity=< 4,  5>"
                .parse::<Star>()
                .expect("Failed to parse"),
            Star {
                pos: Vec2 {
                    x: -39906,
                    y: -49878
                },
                velocity: Vec2 { x: 4, y: 5 }
            }
        );

        assert_eq!(
            "position=< -9838, -29865> velocity=< 1,  3>"
                .parse::<Star>()
                .expect("Failed to parse"),
            Star {
                pos: Vec2 {
                    x: -9838,
                    y: -29865
                },
                velocity: Vec2 { x: 1, y: 3 }
            }
        )
    }

    #[test]
    fn day10_tick() {
        let star = "position=< 7,  6> velocity=<-1, -1>"
            .parse::<Star>()
            .expect("Failed to parse");
        let pos_zero = star.pos_at_t(0);
        assert_eq!(pos_zero.x, 7);
        assert_eq!(pos_zero.y, 6);

        let pos_one = star.pos_at_t(1);
        assert_eq!(pos_one.x, 6);
        assert_eq!(pos_one.y, 5);
    }

    #[test]
    fn day10_part_one() {
        let expected: String = "\n#...#..###
#...#...#.
#...#...#.
#####...#.
#...#...#.
#...#...#.
#...#...#.
#...#..###
"
        .into();

        let actual = part_one(&gen_stars(INPUT));
        assert_eq!(actual, expected);
    }

    #[test]
    fn day10_part_two() {
        assert_eq!(part_two(&gen_stars(INPUT)), 3);
    }

}
