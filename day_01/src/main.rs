use std::fs;

fn main() {
    let contents =
        fs::read_to_string("elf_calories.txt").expect("Something went wrong reading the file");

    let max_calories = get_max_calories(&contents);
    print!("Max calories: {}", max_calories);
}

pub fn get_max_calories(input: &str) -> usize {
    let mut elves = Vec::new();
    let mut sum = 0;

    for line in input.lines() {
        let trimmed = line.trim();

        if trimmed.is_empty() {
            elves.push(sum);
            sum = 0;
        } else {
            sum += trimmed.parse::<usize>().unwrap();
        }
    }

    let max_calories = *elves.iter().max().unwrap();
    max_calories
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "1000
        2000
        3000
        
        4000
        
        5000
        6000
        
        7000
        8000
        9000
        
        10000";

        let max_calories = get_max_calories(input);
        assert_eq!(max_calories, 24000);
    }
}
