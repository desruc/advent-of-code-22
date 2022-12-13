use std::fs;

fn main() {
    let rucksacks =
        fs::read_to_string("rucksacks.txt").expect("Something went wrong reading the file");

    let part_one_answer = part_one(&rucksacks);
    println!("Part one answer: {}", part_one_answer);
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
        println!("{}", line);
        let (first_half, second_half) = split_rucksack_in_half(line.trim());

        println!("{} {}", first_half, second_half);
        let dup = find_duplicate_char(&first_half, &second_half);
        let priority = get_item_priority(dup);

        sum += priority;
    }

    sum
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
}
