const OFFSET_MAJ: u8 = 'a' as u8 - 'A' as u8;

/// Reduces the polymer
fn reduce(input: &[u8]) -> String {
    let mut i = 0;
    let mut j = 1;
    (0..input.len())
        .filter_map(|_| {
            let a = input.get(i);
            let b = input.get(j);
            i += 1;
            j += 1;

            match (a, b) {
                (Some(aa), None) => Some(*aa as char),
                (Some(aa), Some(bb)) if aa.max(bb) - aa.min(bb) == OFFSET_MAJ => {
                    i += 1;
                    j += 1;
                    None
                }
                (Some(aa), Some(_)) => Some(*aa as char),
                _ => None,
            }
        })
        .collect()
}

/// Removes all occurence of a given char and its capitalized version.
/// Said char is specified as u8
fn remove_specified(input: &[u8], spec: u8) -> String {
    input
        .iter()
        .filter_map(|a| {
            if *a != spec && *a + OFFSET_MAJ != spec {
                Some(*a as char)
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
    let mut old = input.to_string();
    loop {
        let new = reduce(old.as_bytes());
        if new.len() == old.len() {
            return new.len();
        }
        old = new;
    }
}

/// Solves part two
#[aoc(day5, part2)]
fn part_two(input: &str) -> usize {
    ('a' as u8..='z' as u8)
        .map(|a| part_one(&remove_specified(input.as_bytes(), a)))
        .min()
        .expect("Could not find minimum")
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn day5_reduce() {
        assert_eq!(reduce("aA".as_bytes()), "".to_string());
        assert_eq!(reduce("abBA".as_bytes()), "aA".to_string());
        assert_eq!(reduce("abAB".as_bytes()), "abAB".to_string());
        assert_eq!(reduce("aabAAB".as_bytes()), "aabAAB".to_string());
    }

    #[test]
    fn day5_reduce_spec() {
        assert_eq!(
            remove_specified("dabAcCaCBAcCcaDA".as_bytes(), 'a' as u8),
            "dbcCCBcCcD".to_string()
        );
        assert_eq!(
            remove_specified("dabAcCaCBAcCcaDA".as_bytes(), 'b' as u8),
            "daAcCaCAcCcaDA".to_string()
        );
        assert_eq!(
            remove_specified("dabAcCaCBAcCcaDA".as_bytes(), 'c' as u8),
            "dabAaBAaDA".to_string()
        );
        assert_eq!(
            remove_specified("dabAcCaCBAcCcaDA".as_bytes(), 'd' as u8),
            "abAcCaCBAcCcaA".to_string()
        );
    }
}
