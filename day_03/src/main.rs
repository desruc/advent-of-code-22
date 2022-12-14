use std::fs;

fn main() {
    let rucksacks =
        fs::read_to_string("rucksacks.txt").expect("Something went wrong reading the file");

    let part_one_answer = part_one(&rucksacks);
    println!("Part one answer: {}", part_one_answer);

    let part_two_answer = part_two(&rucksacks);
    println!("Part two answer: {}", part_two_answer);
}

fn split_rucksack_in_half(rucksack: &str) -> (&str, &str) {
    rucksack.split_at(rucksack.len() / 2)
}

fn find_duplicate_char(first_half: &str, second_half: &str) -> char {
    let mut dup = ' ';

    for c in first_half.chars() {
        if second_half.contains(c) {
            dup = c;
            break;
        }
    }

    dup
}

fn get_item_priority(duplicate: char) -> usize {
    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let position = alphabet.find(duplicate).unwrap();
    position + 1
}

pub fn part_one(input: &str) -> usize {
    let mut sum = 0;

    for line in input.lines() {
        let (first_half, second_half) = split_rucksack_in_half(line.trim());

        let dup = find_duplicate_char(&first_half, &second_half);
        let priority = get_item_priority(dup);

        sum += priority;
    }

    sum
}

fn find_char_present_in_each_string(group: &[String]) -> char {
    let mut dup = ' ';

    for c in group[0].chars() {
        if group[1].contains(c) && group[2].contains(c) {
            dup = c;
            break;
        }
    }

    dup
}

pub fn part_two(input: &str) -> usize {
    let lines: Vec<String> = input
        .lines()
        .map(|x| String::from(x.trim()))
        .collect();

    lines
        .chunks(3)
        .map(|g| {
            let dup = find_char_present_in_each_string(g);
            get_item_priority(dup)
        })
        .reduce(|a, b| a + b)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part_one_works() {
        let sum = part_one(TEST_INPUT);
        assert_eq!(sum, 157);
    }

    #[test]
    fn part_two_works() {
        let sum = part_two(TEST_INPUT);
        assert_eq!(sum, 70);
    }
}
