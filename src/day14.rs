use std::char;

#[aoc(day14, part1)]
pub fn part_one(input: &str) -> String {
    let input: u32 = input.parse().unwrap();

    let mut scores: Vec<usize> = vec![3, 7];
    let mut index_elf_one = 0;
    let mut index_elf_two = 1;

    (0..).find(|_| {
        let sum = scores[index_elf_one] + scores[index_elf_two];
        let sum = &format!("{}", sum);
        let mut digits: Vec<usize> = sum
            .chars()
            .map(|c| c.to_digit(10).expect("Failed to parse digit") as usize)
            .collect();
        scores.append(&mut digits);
        index_elf_one = (index_elf_one + 1 + scores[index_elf_one]) % scores.len();
        index_elf_two = (index_elf_two + 1 + scores[index_elf_two]) % scores.len();
        scores.len() > input as usize + 10
    });

    scores
        .iter()
        .skip(input as usize)
        .take(10)
        .map(|c| char::from_digit(*c as u32, 10).expect("Failed to convert to char"))
        .collect()
}

#[aoc(day14, part2)]
pub fn part_two(input: &str) -> usize {
    let input_size = input.chars().count();
    let mut scores: Vec<usize> = vec![3, 7];
    let mut index_elf_one = 0;
    let mut index_elf_two = 1;
    let mut offset_recipe = 0;

    (0..)
        .find(|_| {
            let sum = scores[index_elf_one] + scores[index_elf_two];
            let sum = &format!("{}", sum);
            let mut digits: Vec<usize> = sum
                .chars()
                .map(|c| c.to_digit(10).expect("Failed to parse digit") as usize)
                .collect();
            if digits.len() == 2 {
                offset_recipe += 1;
            }
            scores.append(&mut digits);
            index_elf_one = (index_elf_one + 1 + scores[index_elf_one]) % scores.len();
            index_elf_two = (index_elf_two + 1 + scores[index_elf_two]) % scores.len();

            match scores.len().checked_sub(input_size + 1) {
                Some(i) => {
                    let i_other = i + 1;
                    let check_i: String = scores
                        .iter()
                        .skip(i)
                        .map(|c| char::from_digit(*c as u32, 10).unwrap())
                        .collect();
                    let check_i_other: String = scores
                        .iter()
                        .skip(i_other)
                        .map(|c| char::from_digit(*c as u32, 10).unwrap())
                        .collect();
                    &check_i == input || &check_i_other == input
                }
                _ => false,
            }
        })
        .unwrap()
        + offset_recipe // Number of times we added 2 recipes
        - 2 // Minus to two recipes from the start
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn day14_part_one() {
        assert_eq!(part_one("9"), "5158916779".to_string());
        assert_eq!(part_one("5"), "0124515891".to_string());
        assert_eq!(part_one("18"), "9251071085".to_string());
        assert_eq!(part_one("2018"), "5941429882".to_string());
    }

    #[test]
    fn day14_part_two() {
        assert_eq!(part_two("51589"), 9);
        assert_eq!(part_two("01245"), 5);
        assert_eq!(part_two("92510"), 18);
        assert_eq!(part_two("59414"), 2018);
    }
}
