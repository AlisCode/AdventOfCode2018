/// Generates the grid
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

/// Solves part one
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

/// Helper function just to garantee that I dont mess up writing different formulas
/// to calculate the indices
fn calc_idx(x: i32, y: i32) -> i32 {
    if x - 1 >= 0 && y - 1 >= 0 {
        return (y - 1) * 300 + x - 1;
    }
    -1
}

/// Generates the summed table
/// https://en.wikipedia.org/wiki/Summed-area_table
fn summed_table(grid: &[i32]) -> Vec<i32> {
    let mut summed_table: Vec<i32> = vec![0; 300 * 300];
    (1..300)
        .map(|y| (1..300).map(move |x| (x, y)))
        .flat_map(|i| i.into_iter())
        .for_each(|(x, y)| {
            let idx: i32 = calc_idx(x, y);
            let idx_left = calc_idx(x - 1, y);
            let idx_top = calc_idx(x, y - 1);
            let idx_top_left = calc_idx(x - 1, y - 1);
            summed_table[idx as usize] = grid[idx as usize]
                + summed_table.get(idx_left as usize).unwrap_or(&0)
                + summed_table.get(idx_top as usize).unwrap_or(&0)
                - summed_table.get(idx_top_left as usize).unwrap_or(&0)
        });
    summed_table
}

/// Extracts the value from the summed area table
/// https://en.wikipedia.org/wiki/Summed-area_table
fn extract_sum(summed: &[i32], x: i32, y: i32, s: i32) -> i32 {
    let a = &summed[calc_idx(x - 1, y - 1) as usize];
    let b = &summed[calc_idx(x + s - 1, y - 1) as usize];
    let c = &summed[calc_idx(x - 1, y + s - 1) as usize];
    let d = &summed[calc_idx(x + s - 1, y + s - 1) as usize];
    d + a - b - c
}

/// Solves part two
#[aoc(day11, part2)]
fn part_two(grid: &[i32]) -> String {
    let summed = summed_table(grid);
    let (_, (x, y, s)) = (1..300)
        .map(|s| {
            (2..300 - s)
                .map(move |x| (2..300 - s).map(move |y| (x, y, s)))
                .flat_map(|i| i.into_iter())
        })
        .flat_map(|i| i.into_iter())
        .map(|(x, y, s)| (extract_sum(&summed, x, y, s), (x, y, s)))
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
    fn day11_summed_table() {
        let grid = gen_grid("18");
        let summed = summed_table(&grid);
        assert_eq!(summed[0], grid[0]);

        let val_check = summed[602];
        let val_summed = grid[0]
            + grid[1]
            + grid[2]
            + grid[300]
            + grid[301]
            + grid[302]
            + grid[600]
            + grid[601]
            + grid[602];
        assert_eq!(val_check, val_summed);

        let extracted = extract_sum(&summed, 90, 269, 16);
        assert_eq!(extracted, 113);
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
