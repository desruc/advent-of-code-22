use std::fs;

fn main() {
    let guide =
        fs::read_to_string("strategy_guide.txt").expect("Something went wrong reading the file");

    let part_one_answer = play_game(&guide);
    println!("Part one answer: {}", part_one_answer);
}

pub fn play_game(strategy_guide: &str) -> i32 {
    let mut game_score = 0;

    // Step through lines and calculate score
    for line in strategy_guide.lines() {
        let mut it = line.split_whitespace();

        // Unwrap the first char
        let opponent_choice = it.next().unwrap().chars().next().unwrap();
        // Unwrap the second char (Each line is always two chars)
        let choice = it.last().unwrap().chars().next().unwrap();

        let round_score = get_round_score(choice, opponent_choice);

        game_score += round_score;

        println!(
            "Round: {:?} vs {:?} | Score: {}",
            get_tool_name(choice),
            get_tool_name(opponent_choice),
            round_score
        );
    }

    println!("Total score: {}", game_score);

    game_score
}

fn get_tool_name(tool: char) -> &'static str {
    match tool {
        'X' | 'A' => "Rock",
        'Y' | 'B' => "Paper",
        'Z' | 'C' => "Scissors",
        _ => "Unknown",
    }
}

pub fn is_win(choice: char, opponent_choice: char) -> bool {
    match choice {
        'Y' => opponent_choice == 'A', // Opponent chooses Rock
        'Z' => opponent_choice == 'B', // Opponent chooses Paper
        'X' => opponent_choice == 'C', // Opponent choose Scissors
        _ => false,
    }
}

fn is_draw(choice: char, opponent_choice: char) -> bool {
    get_tool_name(choice) == get_tool_name(opponent_choice)
}

pub fn get_round_score(choice: char, opponent_choice: char) -> i32 {
    let mut score = 0;

    if is_win(choice, opponent_choice) {
        score += 6;
    } else if is_draw(choice, opponent_choice) {
        score += 3;
    }

    match choice {
        'X' => score += 1,
        'Y' => score += 2,
        'Z' => score += 3,
        _ => (),
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "A Y
    B X
    C Z";

    #[test]
    fn part_one_works() {
        let total_score = play_game(TEST_INPUT);
        assert_eq!(total_score, 15);
    }
}
