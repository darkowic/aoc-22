use std::fs;

const INPUT_FILE_PATH: &str = "input.txt";

const ROCK: &str = "A";
const PAPER: &str = "B";
const SCISSORS: &str = "C";

const LOSE: i32 = 0;
const DRAW: i32 = 3;
const WIN: i32 = 6;

fn normalize_symbols(symbol: &str) -> &str {
    match symbol {
        "X" => ROCK,
        "Y" => PAPER,
        "Z" => SCISSORS,
        _ => panic!("Unexpected symbol"),
    }
}

/**
 * Return true if player A wins
 */
fn rsp_points(player_a: &str, player_b: &str) -> i32 {
    if player_a == player_b {
        return DRAW;
    };
    return match (player_a, player_b) {
        (PAPER, ROCK) => WIN,
        (SCISSORS, PAPER) => WIN,
        (ROCK, SCISSORS) => WIN,
        _ => LOSE,
    };
}

fn my_symbol_to_points(symbol: &str) -> i32 {
    match symbol {
        ROCK => 1,
        PAPER => 2,
        SCISSORS => 3,
        _ => 0,
    }
}

fn choice_based_on_strategy<'a>(strategy: &'a str, opponent: &'a str) -> &'a str {
    match strategy {
        // Loose
        "X" => match opponent {
            ROCK => SCISSORS,
            SCISSORS => PAPER,
            PAPER => ROCK,
            _ => panic!("unexpected symbol"),
        },
        // Win
        "Z" => match opponent {
            SCISSORS => ROCK,
            ROCK => PAPER,
            PAPER => SCISSORS,
            _ => panic!("unexpected symbol"),
        },
        _ => opponent,
    }
}

fn main() {
    let contents =
        fs::read_to_string(INPUT_FILE_PATH).expect("Should have been able to read the file");

    let part1_result = contents
        .split("\n")
        .filter_map(|line| line.split_once(" "))
        .map(|(opponent, me)| {
            let my_choice = normalize_symbols(me);
            return my_symbol_to_points(my_choice) + rsp_points(my_choice, opponent);
        })
        .sum::<i32>();

    println!("What would your total score be if everything goes exactly according to your strategy guide?\nAnswer: {part1_result}");

    let part2_result = contents
        .split("\n")
        .filter_map(|line| line.split_once(" "))
        .map(|(opponent, strategy)| {
            let my_choice = choice_based_on_strategy(strategy, opponent);

            return my_symbol_to_points(my_choice) + rsp_points(my_choice, opponent);
        })
        .sum::<i32>();

    println!("Following the Elf's instructions for the second column, what would your total score be if everything goes exactly according to your strategy guide?\nAnswer: {part2_result}");
}
