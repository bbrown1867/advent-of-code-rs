use super::text_file_to_vec;

const SHAPE_ROCK: u32 = 1;
const SHAPE_PAPER: u32 = 2;
const SHAPE_SCISSORS: u32 = 3;
const RESULT_WIN: u32 = 6;
const RESULT_LOSS: u32 = 0;
const RESULT_DRAW: u32 = 3;

pub fn main() {
    let raw_data = text_file_to_vec("data/day02.txt");

    let score = score_strategy_guide_part1(&raw_data);
    println!("day02 part1: {}", score);

    let score = score_strategy_guide_part2(&raw_data);
    println!("day02 part2: {}", score);
}

fn get_shape(symbol: &str) -> u32 {
    match symbol {
        "A" | "X" => SHAPE_ROCK,
        "B" | "Y" => SHAPE_PAPER,
        "C" | "Z" => SHAPE_SCISSORS,
        _ => panic!("Invalid input"),
    }
}

fn get_outcome(symbol: &str) -> u32 {
    match symbol {
        "X" => RESULT_LOSS,
        "Y" => RESULT_DRAW,
        "Z" => RESULT_WIN,
        _ => panic!("Invalid input"),
    }
}

fn score_hand(player: u32, opponent: u32) -> u32 {
    player
        + if player == opponent {
            RESULT_DRAW
        } else if (player == SHAPE_ROCK && opponent == SHAPE_SCISSORS)
            || (player == SHAPE_PAPER && opponent == SHAPE_ROCK)
            || (player == SHAPE_SCISSORS && opponent == SHAPE_PAPER)
        {
            RESULT_WIN
        } else {
            RESULT_LOSS
        }
}

fn find_hand(outcome: u32, opponent: u32) -> u32 {
    if outcome == RESULT_DRAW {
        opponent
    } else if outcome == RESULT_WIN {
        match opponent {
            SHAPE_ROCK => SHAPE_PAPER,
            SHAPE_PAPER => SHAPE_SCISSORS,
            SHAPE_SCISSORS => SHAPE_ROCK,
            _ => panic!("Invalid input"),
        }
    } else {
        match opponent {
            SHAPE_ROCK => SHAPE_SCISSORS,
            SHAPE_PAPER => SHAPE_ROCK,
            SHAPE_SCISSORS => SHAPE_PAPER,
            _ => panic!("Invalid input"),
        }
    }
}

fn score_strategy_guide_part1(strategy_guide: &Vec<String>) -> u32 {
    let mut score = 0;
    for line in strategy_guide {
        if line.len() == 0 {
            break;
        }
        let mut split = line.split_whitespace();
        let opponent = get_shape(split.next().unwrap());
        let player = get_shape(split.next().unwrap());
        score += score_hand(player, opponent);
    }

    score
}

fn score_strategy_guide_part2(strategy_guide: &Vec<String>) -> u32 {
    let mut score = 0;
    for line in strategy_guide {
        if line.len() == 0 {
            break;
        }
        let mut split = line.split_whitespace();
        let opponent = get_shape(split.next().unwrap());
        let outcome = get_outcome(split.next().unwrap());
        let player = find_hand(outcome, opponent);
        score += score_hand(player, opponent);
    }

    score
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_test_input() -> Vec<String> {
        vec!["A Y", "B X", "C Z"]
            .iter()
            .map(|x| x.to_string())
            .collect()
    }

    #[test]
    fn test_score_strategy_guide_part1() {
        assert_eq!(score_strategy_guide_part1(&get_test_input()), 15);
    }

    #[test]
    fn test_score_strategy_guide_part2() {
        assert_eq!(score_strategy_guide_part2(&get_test_input()), 12);
    }
}
