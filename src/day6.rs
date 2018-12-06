use fnv::FnvHashMap;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Point2 {
    x: i32,
    y: i32,
}

impl Point2 {
    pub fn new(x: i32, y: i32) -> Self {
        Point2 { x, y }
    }

    pub fn distance(&self, other: &Self) -> i32 {
        self.x.max(other.x) - self.x.min(other.x) + self.y.max(other.y) - self.y.min(other.y)
    }
}

impl FromStr for Point2 {
    type Err = ParseIntError;

    fn from_str(input: &str) -> Result<Point2, Self::Err> {
        let parts: Vec<&str> = input.trim().split(",").collect();
        let x: i32 = parts[0].trim().parse()?;
        let y: i32 = parts[1].trim().parse()?;
        Ok(Point2::new(x, y))
    }
}

fn bounding_box(input: &[Point2]) -> (i32, i32, i32, i32) {
    input.iter().fold((0, 0, 0, 0), |acc, i| {
        let left = acc.0.min(i.x);
        let top = acc.1.min(i.x);
        let right = acc.2.min(i.y);
        let bottom = acc.3.max(i.y);
        (left, top, right, bottom)
    })
}

#[aoc_generator(day6)]
pub fn generator(input: &str) -> Vec<Point2> {
    input
        .lines()
        .map(|l| l.parse().expect("Failed to parse Point2"))
        .collect()
}

pub fn part_one(input: &[Point2]) -> i32 {
    let bbox = bounding_box(input);
    let final_bounds = (bbox.0 - 1000, bbox.1 - 1000, bbox.2 + 1000, bbox.3 + 1000);

    let input_filtered: Vec<Point2> = input
        .iter()
        .filter_map(|p| {
            if p.x != bbox.0 && p.x != bbox.2 && p.y != bbox.1 && p.y != bbox.3 {
                Some(p.clone())
            } else {
                None
            }
        })
        .collect();

    let mut hash_map: FnvHashMap<Point2, i32> = FnvHashMap::default();
    (final_bounds.0..=final_bounds.2)
        .map(|x| (final_bounds.1..=final_bounds.3).map(move |y| Point2::new(x, y)))
        .flat_map(|i| i.into_iter())
        .for_each(|i| {
            let (point, _, count) = input_filtered.iter().map(|p| (p, p.distance(&i))).fold(
                (Point2::new(0, 0), 0, 0),
                |acc, i| match i.1 {
                    x if x < acc.1 => (i.0.clone(), i.1, 1),
                    x if x == acc.1 => (i.0.clone(), i.1, acc.2 + 1),
                    _ => acc,
                },
            );
            if count == 1 {
                let p_val = hash_map.entry(point).or_insert(0);
                *p_val += 1;
            }
        });

    *hash_map.values().max().expect("Could not find max value")
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn day6_parse() {
        assert_eq!(
            "1, 5".parse::<Point2>().expect("Failed to parse"),
            Point2::new(1, 5)
        );
    }

    #[test]
    fn day6_part_one() {
        let input: &str = "1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9";
        let points = generator(input);
        assert_eq!(part_one(&points), 17);
    }
}
