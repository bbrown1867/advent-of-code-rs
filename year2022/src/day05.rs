use super::text_file_to_vec;
use std::collections::BTreeMap;
use std::str::FromStr;

pub fn main() {
    let raw_data = text_file_to_vec("data/day05.txt");

    let (mut stack, steps) = parse_stack_and_steps(&raw_data);
    let result = apply_steps_and_get_top_crates(&mut stack, steps, false);
    assert_eq!(result, "CNSZFDVLJ");

    let (mut stack, steps) = parse_stack_and_steps(&raw_data);
    let result = apply_steps_and_get_top_crates(&mut stack, steps, true);
    assert_eq!(result, "QNDWLMGNS");
}

type Stack = BTreeMap<u32, Vec<char>>;

#[derive(Debug)]
struct Step {
    src: u32,
    dst: u32,
    cnt: u32,
}

fn apply_steps_and_get_top_crates(
    stack: &mut Stack,
    steps: Vec<Step>,
    move_together: bool,
) -> String {
    for step in steps {
        // Pop "step.cnt" chars from crates[src] into a temp variable
        let mut popped_crates = String::new();
        for _ in 0..step.cnt {
            match stack.get_mut(&step.src).unwrap().pop() {
                Some(c) => popped_crates.push(c),
                // "step.src" has no more elements, just skip and move on
                None => (),
            }
        }

        // Push the chars of the temp variable onto crates[dst]
        if move_together {
            for c in popped_crates.chars().rev() {
                stack.get_mut(&step.dst).unwrap().push(c);
            }
        } else {
            for c in popped_crates.chars() {
                stack.get_mut(&step.dst).unwrap().push(c);
            }
        }
    }

    // Get the top crate for each stack
    let mut result = String::new();
    for (_, crates) in stack {
        if crates.len() > 0 {
            result.push(crates.pop().unwrap());
        }
    }

    result
}

fn parse_stack_and_steps(raw_data: &Vec<String>) -> (Stack, Vec<Step>) {
    let mut stack = Stack::new();
    let mut steps = Vec::<Step>::new();

    let mut raw_data = raw_data.iter();

    // Parse stack: Note that I've inserted dummy crates ("[x]") to simplify parsing
    let mut line = raw_data.next().unwrap();
    while line.chars().nth(0).unwrap() == '[' {
        // Iterate over each stack
        for (stack_num, crate_val) in line.split_whitespace().enumerate() {
            // Crate syntax is "[A]", so second char is the label
            let crate_val = crate_val.chars().nth(1).unwrap();
            if crate_val == 'x' {
                // Dummy value, skip
                continue;
            } else {
                // Otherwise insert into the stack
                match stack.get_mut(&((stack_num + 1) as u32)) {
                    Some(v) => v.push(crate_val),
                    None => {
                        let mut new_stack = Vec::new();
                        new_stack.push(crate_val);
                        stack.insert((stack_num + 1) as u32, new_stack);
                    }
                }
            }
        }

        line = raw_data.next().unwrap();
    }

    // Skip line with stack numbers
    let _ = raw_data.next().unwrap();

    // Skip empty line
    let mut line = raw_data.next();

    // Parse steps
    loop {
        match line {
            Some(step) => {
                // move {cnt} from {src} to {dst}
                let mut parsed_step = step.split_whitespace();
                let _ = parsed_step.next().unwrap();
                let cnt = u32::from_str(parsed_step.next().unwrap()).unwrap();
                let _ = parsed_step.next().unwrap();
                let src = u32::from_str(parsed_step.next().unwrap()).unwrap();
                let _ = parsed_step.next().unwrap();
                let dst = u32::from_str(parsed_step.next().unwrap()).unwrap();
                steps.push(Step { src, dst, cnt });
            }
            None => break,
        }

        line = raw_data.next();
    }

    // Because of the way we parse, all stacks are backwards. We want "bottom"
    // elements to be at index 0 of the vector.
    for (_, crates) in &mut stack {
        crates.reverse();
    }

    (stack, steps)
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_test_input() -> Vec<String> {
        vec![
            "[x] [D] [x]",
            "[N] [C] [x] ",
            "[Z] [M] [P]",
            " 1   2   3 ",
            "",
            "move 1 from 2 to 1",
            "move 3 from 1 to 3",
            "move 2 from 2 to 1",
            "move 1 from 1 to 2",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect()
    }

    #[test]
    fn test_parse_stack_and_steps() {
        let (stack, steps) = parse_stack_and_steps(&get_test_input());
        for (stack_num, crates) in stack {
            println!("{}, {:?}", stack_num, crates);
        }

        for step in steps {
            println!("{:?}", step);
        }
    }

    #[test]
    fn test_apply_steps_and_get_top_crates_part1() {
        let (mut stack, steps) = parse_stack_and_steps(&get_test_input());
        let result = apply_steps_and_get_top_crates(&mut stack, steps, false);
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn test_apply_steps_and_get_top_crates_part2() {
        let (mut stack, steps) = parse_stack_and_steps(&get_test_input());
        let result = apply_steps_and_get_top_crates(&mut stack, steps, true);
        assert_eq!(result, "MCD");
    }
}
