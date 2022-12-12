use std::fs;

fn main() {
    let part_one_answer = part_one();
    println!("Part one answer: {}", part_one_answer);

    let part_two_answer = part_two();
    println!("Part two answer: {}", part_two_answer);
}

fn get_elves(input: &str) -> Vec<usize> {
    let mut elves = Vec::new();
    let mut sum = 0;

    let mut it = input.lines().peekable();

    while let Some(line) = it.next() {
        let trimmed = line.clone().trim();

        if trimmed.is_empty() {
            elves.push(sum);

            sum = 0;
        } else {
            sum += trimmed.parse::<usize>().unwrap();

            // If it's the last line, push the sum before we exit the loop
            if it.peek().is_none() {
                elves.push(sum);
            }
        }
    }

    elves
}

fn get_elf_with_max_calories(elves: Vec<usize>) -> usize {
    *elves.iter().max().unwrap()
}

fn get_top_three_elves_with_most_calories(elves: Vec<usize>) -> Vec<usize> {
    let mut elves = elves;
    elves.sort();
    elves.reverse();
    elves.truncate(3);
    elves
}

pub fn part_one() -> usize {
    let contents =
        fs::read_to_string("elf_calories.txt").expect("Something went wrong reading the file");

    let elves = get_elves(&contents);

    get_elf_with_max_calories(elves)
}

pub fn part_two() -> usize {
    let contents =
        fs::read_to_string("elf_calories.txt").expect("Something went wrong reading the file");

    let elves = get_elves(&contents);

    let top_three_elves = get_top_three_elves_with_most_calories(elves);
    top_three_elves.iter().sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn part_one_works() {
        let elves = get_elves(&TEST_INPUT);

        let max_calories = get_elf_with_max_calories(elves);

        assert_eq!(max_calories, 24000);
    }

    #[test]
    fn part_two_works() {
        let elves = get_elves(&TEST_INPUT);

        let top_three_elves = get_top_three_elves_with_most_calories(elves);

        let sum = top_three_elves.iter().sum::<usize>();

        assert_eq!(sum, 45000);
    }
}
