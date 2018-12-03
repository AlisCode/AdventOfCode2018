use edit_distance::edit_distance;
use fnv::FnvHashMap;

/// Handles the line so we know if we found two or three times the same character
/// Returns either (0,0) / (1,0) / (0,1) / (1,1)
fn treat_line(input: &str) -> (usize, usize) {
    let mut map: FnvHashMap<char, usize> = FnvHashMap::default();
    input.chars().for_each(|a| *map.entry(a).or_insert(0) += 1);

    let twos = match map.values().any(|&v| v == 2usize) {
        true => 1,
        _ => 0,
    };
    let threes = match map.values().any(|&v| v == 3usize) {
        true => 1,
        _ => 0,
    };
    (twos, threes)
}

/// Gets the common part of the two given &str into a String
fn get_common(a: &str, b: &str) -> String {
    a.chars()
        .zip(b.chars())
        .filter_map(|(a, b)| if a == b { Some(a) } else { None })
        .collect()
}

/// Solves part one
#[aoc(day2, part1)]
fn part_one(input: &str) -> usize {
    let numbers: (usize, usize) = input
        .lines()
        .map(|l| treat_line(l))
        .fold((0, 0), |acc, i| (acc.0 + i.0, acc.1 + i.1));
    numbers.0 * numbers.1
}

/// Solves part two
#[aoc(day2, part2)]
fn part_two(input: &str) -> String {
    for src in input.lines() {
        let one_diff: Option<&str> = input.lines().filter(|l| edit_distance(src, l) == 1).next();
        match one_diff {
            Some(s) => return get_common(src, s),
            None => (),
        }
    }
    unreachable!()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn day2_part1_treatline() {
        assert_eq!(treat_line("abcdef"), (0, 0));
        assert_eq!(treat_line("bababc"), (1, 1));
        assert_eq!(treat_line("abbcde"), (1, 0));
        assert_eq!(treat_line("abcccd"), (0, 1));
        assert_eq!(treat_line("aabcdd"), (1, 0));
        assert_eq!(treat_line("abcdee"), (1, 0));
        assert_eq!(treat_line("ababab"), (0, 1));
    }

    #[test]
    fn day2_part1_full() {
        let input: &str = "abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab";

        assert_eq!(part_one(input), 4 * 3)
    }

    #[test]
    fn day2_part2_get_common() {
        let a = "fghij";
        let b = "fguij";
        assert_eq!(get_common(a, b), "fgij")
    }

    #[test]
    fn day2_part2_full() {
        let input: &str = "abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz";
        assert_eq!(part_two(input), "fgij")
    }

}
