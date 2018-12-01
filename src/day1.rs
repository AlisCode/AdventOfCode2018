use fnv::FnvHashSet;

#[aoc_generator(day1)]
fn generator_input(input: &str) -> Vec<i32> {
    input.lines().map(|a| a.parse::<i32>().unwrap()).collect()
}

#[aoc(day1, part1)]
fn part_one(input: &[i32]) -> i32 {
    input.into_iter().sum()
}

#[aoc(day1, part2)]
fn part_two(input: &[i32]) -> i32 {
    let mut seen = FnvHashSet::default();
    let mut sum = 0;
    seen.insert(0);
    let _ = input
        .into_iter()
        .cycle()
        .filter(|&a| {
            sum += a;
            !seen.insert(sum.clone())
        })
        .next();
    sum
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn day1_generator() {
        // Awful formatting, thanks rustfmt I guess...
        assert_eq!(
            generator_input(
                "+1
+1
+1"
            ),
            &[1, 1, 1]
        );

        assert_eq!(
            generator_input(
                "+1
+1
-2"
            ),
            &[1, 1, -2]
        );
    }

    #[test]
    fn day1_part1() {
        assert_eq!(part_one(&[1, -2, 3, 1]), 3);
        assert_eq!(part_one(&[1, 1, 1]), 3);
        assert_eq!(part_one(&[1, 1, -2]), 0);
        assert_eq!(part_one(&[-1, -2, -3]), -6);
    }

    #[test]
    fn day1_part2() {
        assert_eq!(part_two(&[3, 3, 4, -2, -4]), 10);
        assert_eq!(part_two(&[-6, 3, 8, 5, -6]), 5);
        assert_eq!(part_two(&[7, 7, -2, -7, -4]), 14);
        assert_eq!(part_two(&[1, -2, 3, 1]), 2);
        assert_eq!(part_two(&[1, -1]), 0);
    }
}
