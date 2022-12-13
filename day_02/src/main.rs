use std::fs;

#[derive(PartialEq)]
pub enum Tool {
    Rock,
    Paper,
    Scissors,
    Unknown,
}

pub enum Outcome {
    Win,
    Draw,
    Lose,
    Unknown,
}

fn main() {
    let guide =
        fs::read_to_string("strategy_guide.txt").expect("Something went wrong reading the file");

    let part_one_answer = part_one(&guide);
    println!("Part one answer: {}", part_one_answer);

    let part_two_answer = part_two(&guide);
    println!("Part two answer: {}", part_two_answer);
}

fn get_tool_from_char(tool_char: char) -> Tool {
    match tool_char {
        'X' | 'A' => Tool::Rock,
        'Y' | 'B' => Tool::Paper,
        'Z' | 'C' => Tool::Scissors,
        _ => Tool::Unknown,
    }
}

fn get_player_choice_from_outcome(opponent_choice: &Tool, outcome: Outcome) -> Tool {
    match outcome {
        Outcome::Win => match opponent_choice {
            Tool::Rock => Tool::Paper,
            Tool::Paper => Tool::Scissors,
            Tool::Scissors => Tool::Rock,
            _ => Tool::Unknown,
        },
        Outcome::Draw => match opponent_choice {
            Tool::Rock => Tool::Rock,
            Tool::Paper => Tool::Paper,
            Tool::Scissors => Tool::Scissors,
            _ => Tool::Unknown,
        },
        Outcome::Lose => match opponent_choice {
            Tool::Rock => Tool::Scissors,
            Tool::Paper => Tool::Rock,
            Tool::Scissors => Tool::Paper,
            _ => Tool::Unknown,
        },
        _ => Tool::Unknown,
    }
}

fn get_outcome_from_char(outcome_char: char) -> Outcome {
    match outcome_char {
        'Z' => Outcome::Win,
        'Y' => Outcome::Draw,
        'X' => Outcome::Lose,
        _ => Outcome::Unknown,
    }
}

pub fn get_round_score(player_choice: Tool, opponent_choice: Tool) -> i32 {
    let mut score = 0;

    if is_win(&player_choice, &opponent_choice) {
        score += 6;
    } else if opponent_choice == player_choice {
        score += 3;
    }

    match player_choice {
        Tool::Rock => score += 1,
        Tool::Paper => score += 2,
        Tool::Scissors => score += 3,
        _ => (),
    }

    score
}

pub fn is_win(choice: &Tool, opponent_choice: &Tool) -> bool {
    match opponent_choice {
        Tool::Rock => matches!(choice, Tool::Paper),
        Tool::Paper => matches!(choice, Tool::Scissors),
        Tool::Scissors => matches!(choice, Tool::Rock),
        _ => false,
    }
}

pub fn part_one(strategy_guide: &str) -> i32 {
    let mut game_score = 0;

    // Step through lines and calculate score
    for line in strategy_guide.lines() {
        let mut it = line.split_whitespace();

        // Unwrap each char (each line is always two chars)
        let opponent_choice_char = it.next().unwrap().chars().next().unwrap();
        let player_choice = it.last().unwrap().chars().next().unwrap();

        let opponent_choice = get_tool_from_char(opponent_choice_char);
        let player_choice = get_tool_from_char(player_choice);

        let round_score = get_round_score(player_choice, opponent_choice);

        game_score += round_score;
    }

    game_score
}

pub fn part_two(guide: &str) -> i32 {
    let mut game_score = 0;

    // Step through lines and calculate score
    for line in guide.lines() {
        let mut it = line.split_whitespace();

        // Unwrap each char (each line is always two chars)
        let opponent_choice_char = it.next().unwrap().chars().next().unwrap();
        let outcome_char = it.last().unwrap().chars().next().unwrap();

        let opponent_choice = get_tool_from_char(opponent_choice_char);
        let outcome = get_outcome_from_char(outcome_char);
        let player_choice = get_player_choice_from_outcome(&opponent_choice, outcome);

        let round_score = get_round_score(player_choice, opponent_choice);

        game_score += round_score;
    }

    game_score
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "A Y
    B X
    C Z";

    #[test]
    fn part_one_works() {
        let total_score = part_one(TEST_INPUT);
        assert_eq!(total_score, 15);
    }

    #[test]
    fn part_two_works() {
        let total_score = part_two(TEST_INPUT);
        assert_eq!(total_score, 12);
    }
}
