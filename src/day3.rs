use std::num::ParseIntError;
use std::str::FromStr;

pub struct Claim {
    pub id: u32,
    pub coords: (u32, u32),
    pub area: (u32, u32),
}

impl Claim {
    pub fn contains(&self, c: (u32, u32)) -> bool {
        c.0 >= self.coords.0
            && c.1 >= self.coords.1
            && c.0 < self.coords.0 + self.area.0
            && c.1 < self.coords.1 + self.area.1
    }

    pub fn contact(&self, other: &Self) -> bool {
        !(self.coords.0 > other.coords.0 + other.area.0 - 1
            || self.coords.0 + self.area.0 - 1 < other.coords.0
            || self.coords.1 > other.coords.1 + other.area.1 - 1
            || self.coords.1 + self.area.1 - 1 < other.coords.1)
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

fn extract_borders(claims: &[Claim]) -> (u32, u32, u32, u32) {
    let first = &claims[0];
    claims.iter().fold(
        (
            first.coords.0,
            first.coords.1,
            first.coords.0 + first.area.0,
            first.coords.1 + first.area.1,
        ),
        |acc, i| {
            let x = acc.0.min(i.coords.0);
            let y = acc.1.min(i.coords.1);
            let x_max = acc.2.max(i.coords.0 + i.area.0);
            let y_max = acc.3.max(i.coords.1 + i.area.1);
            (x, y, x_max, y_max)
        },
    )
}

#[aoc(day3, part1)]
fn part_one(input: &[Claim]) -> usize {
    let squares = extract_borders(input);
    (squares.0..=squares.2)
        .map(|x| (squares.1..=squares.3).map(move |y| (x, y)))
        .flat_map(|i| i.into_iter())
        .filter(|coords| input.iter().filter(|c| c.contains(*coords)).count() >= 2)
        .count()
}

#[aoc(day3, part2)]
fn part_two(input: &[Claim]) -> u32 {
    let lone_claim = input
        .iter()
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
    fn day3_contains() {
        let claims = input_gen("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2");
        let borders = extract_borders(&claims);
        assert_eq!(borders.0, 1);
        assert_eq!(borders.1, 1);
        assert_eq!(borders.2, 7);
        assert_eq!(borders.3, 7);

        let claim = &claims[0];
        assert!(!claim.contains((0, 0)));
        assert!(claim.contains((1, 3)));
        assert!(claim.contains((4, 6)));
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
