/// Reduces the polymer
fn reduce(input: &str) -> String {
    input
        .chars()
        .fold("".to_string(), |mut acc, i| match acc.pop() {
            Some(ii) if (ii as u8) ^ (i as u8) != 32 => {
                acc.push(ii);
                acc.push(i);
                acc
            }
            None => {
                acc.push(i);
                acc
            }
            _ => acc,
        })
}

/// Removes all occurence of a given char and its capitalized version.
/// Said char is specified as u8
fn remove_specified(input: &str, spec: u8) -> String {
    input
        .chars()
        .filter_map(|a| {
            if a as u8 != spec && a as u8 + 32 != spec {
                Some(a)
            } else {
                None
            }
        })
        .collect()
}

/// Today's input was not trimmed!
#[aoc_generator(day5)]
fn generator(input: &str) -> String {
    input.trim().to_string()
}

/// Solves part one
#[aoc(day5, part1)]
fn part_one(input: &str) -> usize {
    reduce(input).len()
}

/// Solves part two
#[aoc(day5, part2)]
fn part_two(input: &str) -> usize {
    ('a' as u8..='z' as u8)
        .map(|a| reduce(&remove_specified(input, a)).len())
        .min()
        .expect("Could not find minimum")
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn day5_reduce() {
        assert_eq!(reduce("aA"), "".to_string());
        assert_eq!(reduce("abBA"), "".to_string());
        assert_eq!(reduce("abAB"), "abAB".to_string());
        assert_eq!(reduce("aabAAB"), "aabAAB".to_string());
    }

    #[test]
    fn day5_reduce_spec() {
        assert_eq!(
            remove_specified("dabAcCaCBAcCcaDA", 'a' as u8),
            "dbcCCBcCcD".to_string()
        );
        assert_eq!(
            remove_specified("dabAcCaCBAcCcaDA", 'b' as u8),
            "daAcCaCAcCcaDA".to_string()
        );
        assert_eq!(
            remove_specified("dabAcCaCBAcCcaDA", 'c' as u8),
            "dabAaBAaDA".to_string()
        );
        assert_eq!(
            remove_specified("dabAcCaCBAcCcaDA", 'd' as u8),
            "abAcCaCBAcCcaA".to_string()
        );
    }
}
