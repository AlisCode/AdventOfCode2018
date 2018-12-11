#[aoc_generator(day11)]
fn gen_grid(input: &str) -> Vec<i32> {
    let grid_serial: i32 = input.parse::<i32>().expect("Failed to get serial number");
    (1..=300)
        .map(|y| (1..=300).map(move |x| (x, y)))
        .flat_map(|i| i.into_iter())
        .map(|(x, y)| {
            let rack_id = x + 10;
            let mut power_level = rack_id * y;
            power_level += grid_serial;
            power_level *= rack_id;
            power_level /= 100;
            power_level %= 10;
            power_level - 5
        })
        .collect()
}

#[aoc(day11, part1)]
fn part_one(input: &[i32]) -> String {
    let (_, coords) = (1..=297)
        .map(|y| (1..=297).map(move |x| (x, y)))
        .flat_map(|i| i.into_iter())
        .map(|(x, y)| {
            (
                (x..=x + 2)
                    .map(|xx| {
                        (y..=y + 2).map(move |yy| {
                            let idx = (yy - 1) * 300 + xx;
                            input[idx]
                        })
                    })
                    .flat_map(|i| i.into_iter())
                    .sum::<i32>(),
                (x + 1, y),
            )
        })
        .max_by_key(|i| i.0)
        .expect("Failed to find min");

    format!("{},{}", coords.0, coords.1)
}

#[aoc(day11, part2)]
fn part_two(input: &[i32]) -> String {
    let (_, (x, y, s)) = (1..300)
        .map(|s| {
            (300 - s..300)
                .map(move |x| (300 - s..300).map(move |y| (x, y, s)))
                .flat_map(|i| i.into_iter())
        })
        .flat_map(|i| i.into_iter())
        .map(|(x, y, s)| {
            (
                (x..300)
                    .map(|xx| (y..300).map(move |yy| (xx, yy)))
                    .flat_map(|i| i.into_iter())
                    .map(|(xx, yy)| {
                        let idx = (yy - 1) * 300 + xx;
                        input[idx]
                    })
                    .sum::<i32>(),
                (x + 1, y, s),
            )
        })
        .max_by_key(|i| i.0)
        .expect("Failed to find max");
    format!("{},{},{}", x, y, s)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn day11_power_level() {
        let grid = gen_grid("57");
        assert_eq!(grid[(79 - 1) * 300 + 122], -5);
    }

    #[test]
    fn day11_part_one() {
        assert_eq!(part_one(&gen_grid("18")), "33,45".to_string());
        assert_eq!(part_one(&gen_grid("42")), "21,61".to_string());
    }

    #[test]
    fn day11_part_two() {
        assert_eq!(part_two(&gen_grid("18")), "90,269,16".to_string());
        assert_eq!(part_two(&gen_grid("42")), "232,251,12".to_string());
    }

}
