use fnv::FnvHashSet;
use std::num::ParseIntError;
use std::str::FromStr;

pub struct Claim {
    pub id: u32,
    pub coords: (u32, u32),
    pub area: (u32, u32),
}

impl Claim {
    pub fn contact(&self, other: &Self) -> bool {
        !(self.coords.0 > other.coords.0 + other.area.0 - 1
            || self.coords.0 + self.area.0 - 1 < other.coords.0
            || self.coords.1 > other.coords.1 + other.area.1 - 1
            || self.coords.1 + self.area.1 - 1 < other.coords.1)
    }

    pub fn overlap(&self, other: &Self) -> impl Iterator<Item = (u32, u32)> {
        // Calc the common rectangle
        let x_min = self.coords.0.max(other.coords.0);
        let x_max = (self.coords.0 + self.area.0).min(other.coords.0 + other.area.0);
        let y_min = self.coords.1.max(other.coords.1);
        let y_max = (self.coords.1 + self.area.1).min(other.coords.1 + other.area.1);

        // Return an iterator
        (x_min..x_max)
            .map(move |x| (y_min..y_max).map(move |y| (x, y)))
            .flat_map(|i| i.into_iter())
    }
}

impl FromStr for Claim {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let first: Vec<&str> = s.split("@").collect();
        let id = first[0].trim().replace("#", "").parse::<u32>()?;
        let second: Vec<&str> = first[1].split(":").collect();
        let coords_val: Vec<&str> = second[0].split(",").collect();
        let area_val: Vec<&str> = second[1].split("x").collect();

        let coords = (
            coords_val[0].trim().parse::<u32>()?,
            coords_val[1].trim().parse::<u32>()?,
        );
        let area = (
            area_val[0].trim().parse::<u32>()?,
            area_val[1].trim().parse::<u32>()?,
        );

        Ok(Claim { id, coords, area })
    }
}

#[aoc_generator(day3)]
fn input_gen(input: &str) -> Vec<Claim> {
    input
        .lines()
        .map(|l| l.parse().expect(&format!("Failed to parse claim {}", l)))
        .collect()
}

#[aoc(day3, part1)]
fn part_one(input: &[Claim]) -> usize {
    input
        .iter()
        .flat_map(|c| {
            input
                .iter()
                .filter(move |cc| cc.id != c.id)
                .flat_map(move |cc| c.overlap(cc))
        })
        .collect::<FnvHashSet<(u32, u32)>>()
        .len()
}

#[aoc(day3, part2)]
fn part_two(input: &[Claim]) -> u32 {
    let lone_claim = input
        .into_iter()
        .filter(|c| !input.iter().any(|cc| cc.id != c.id && c.contact(cc)))
        .next()
        .expect("Could not find lone claim");

    lone_claim.id
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn day3_parse() {
        let claim: Claim = "#123 @ 3,2: 5x4".parse().expect("Failed to parse");
        assert_eq!(claim.id, 123);
        assert_eq!(claim.coords, (3, 2));
        assert_eq!(claim.area, (5, 4));
    }

    #[test]
    fn day3_contact() {
        let claims = input_gen("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2");
        assert!(&claims[0].contact(&claims[1]));
        assert!(&claims[1].contact(&claims[0]));
        assert!(!&claims[1].contact(&claims[2]));
        assert!(!&claims[2].contact(&claims[1]));
        assert!(!&claims[0].contact(&claims[2]));
        assert!(!&claims[2].contact(&claims[0]));

        let overlap: Vec<(u32, u32)> = claims[0].overlap(&claims[1]).collect();
        assert_eq!(overlap.len(), 4);
        assert_eq!(overlap[0], (3, 3));
        assert_eq!(overlap[1], (3, 4));
        assert_eq!(overlap[2], (4, 3));
        assert_eq!(overlap[3], (4, 4));
    }

    #[test]
    fn day3_part1() {
        let claims = input_gen("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2");
        assert_eq!(part_one(&claims), 4);
    }

    #[test]
    fn day3_part2() {
        let claims = input_gen("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2");
        assert_eq!(part_two(&claims), 3);
    }

}
