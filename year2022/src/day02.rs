use super::text_file_to_vec;
use std::str::FromStr;

pub fn main() {
    let raw_data = text_file_to_vec("data/day02.txt");

    let score = score_strategy_guide(&raw_data, true);
    println!("day02 part1: {}", score);

    let score = score_strategy_guide(&raw_data, false);
    println!("day02 part2: {}", score);
}

type Score = u32;

#[derive(Clone, Copy, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone, Copy, PartialEq)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Shape {
    fn new(val: Score) -> Self {
        match val {
            1 => Shape::Rock,
            2 => Shape::Paper,
            3 => Shape::Scissors,
            // Trick to allow subtracting 1 to work when finding losing shape
            0 => Shape::Scissors,
            _ => panic!("Invalid value"),
        }
    }

    fn value(&self) -> Score {
        match &self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

impl FromStr for Shape {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Shape::Rock),
            "B" | "Y" => Ok(Shape::Paper),
            "C" | "Z" => Ok(Shape::Scissors),
            _ => Err(()),
        }
    }
}

impl Outcome {
    fn value(&self) -> Score {
        match &self {
            Outcome::Win => 6,
            Outcome::Loss => 0,
            Outcome::Draw => 3,
        }
    }
}

impl FromStr for Outcome {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Loss),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(()),
        }
    }
}

fn score_hand(player: Shape, opponent: Shape) -> Score {
    player.value()
        + if player == opponent {
            Outcome::Draw.value()
        } else if ((player.value() % 3) + 1) == opponent.value() {
            Outcome::Loss.value()
        } else {
            Outcome::Win.value()
        }
}

fn find_hand(outcome: Outcome, opponent: Shape) -> Shape {
    if outcome == Outcome::Draw {
        opponent
    } else if outcome == Outcome::Win {
        Shape::new((opponent.value() % 3) + 1)
    } else {
        Shape::new(opponent.value() - 1)
    }
}

fn score_strategy_guide(strategy_guide: &Vec<String>, part1: bool) -> Score {
    let mut score = 0;
    for line in strategy_guide {
        if line.len() == 0 {
            break;
        }
        let mut split = line.split_whitespace();
        let opponent = Shape::from_str(split.next().unwrap()).unwrap();
        let player = if part1 {
            Shape::from_str(split.next().unwrap()).unwrap()
        } else {
            let outcome = Outcome::from_str(split.next().unwrap()).unwrap();
            find_hand(outcome, opponent)
        };
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
        assert_eq!(score_strategy_guide(&get_test_input(), true), 15);
    }

    #[test]
    fn test_score_strategy_guide_part2() {
        assert_eq!(score_strategy_guide(&get_test_input(), false), 12);
    }
}
