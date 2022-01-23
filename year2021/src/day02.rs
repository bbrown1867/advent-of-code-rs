use super::text_file_to_vec;

use std::str::FromStr;

pub fn main() {
    let raw_data = text_file_to_vec("data/day02.txt");

    let mut sub1 = Submarine::new(0, 0, 0, true);
    sub1.drive(&raw_data);
    println!("day02 part1: {}", sub1.get_product());

    let mut sub2 = Submarine::new(0, 0, 0, false);
    sub2.drive(&raw_data);
    println!("day02 part2: {}", sub2.get_product());
}

#[derive(PartialEq)]
enum Motion {
    Up,
    Down,
    Forward,
}

impl FromStr for Motion {
    type Err = ();

    fn from_str(input: &str) -> Result<Motion, Self::Err> {
        match input {
            "up" => Ok(Motion::Up),
            "down" => Ok(Motion::Down),
            "forward" => Ok(Motion::Forward),
            _ => Err(()),
        }
    }
}

struct Submarine<T> {
    depth: T,
    position: T,
    aim: T,
    apply_command: fn(&mut Submarine<T>, Motion, T),
}

impl<T> Submarine<T>
where
    T: Copy
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>,
{
    fn new(depth: T, position: T, aim: T, part1: bool) -> Self {
        let apply_command = if part1 {
            Self::apply_command_part1
        } else {
            Self::apply_command_part2
        };

        Submarine {
            depth,
            position,
            aim,
            apply_command,
        }
    }

    fn apply_command_part1(&mut self, motion: Motion, value: T) {
        match motion {
            Motion::Up => self.depth = self.depth - value,
            Motion::Down => self.depth = self.depth + value,
            Motion::Forward => self.position = self.position + value,
        };
    }

    fn apply_command_part2(&mut self, motion: Motion, value: T) {
        match motion {
            Motion::Up => self.aim = self.aim - value,
            Motion::Down => self.aim = self.aim + value,
            Motion::Forward => {
                self.position = self.position + value;
                self.depth = self.depth + self.aim * value;
            }
        }
    }

    fn drive(&mut self, command_list: &Vec<String>)
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        for command in command_list {
            // Parse the command string
            let mut iter = command.split_whitespace();
            let motion = Motion::from_str(iter.next().unwrap()).unwrap();
            let value: T = iter.next().unwrap().parse().unwrap();

            (self.apply_command)(self, motion, value);
        }
    }

    pub fn get_product(&self) -> T {
        self.depth * self.position
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_test_input() -> Vec<String> {
        vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect()
    }

    #[test]
    fn test_submarine_part1() {
        let mut sub = Submarine::new(0, 0, 0, true);
        sub.drive(&get_test_input());
        assert_eq!(sub.get_product(), 150);
    }

    #[test]
    fn test_submarine_part2() {
        let mut sub = Submarine::new(0, 0, 0, false);
        sub.drive(&get_test_input());
        assert_eq!(sub.get_product(), 900);
    }
}
