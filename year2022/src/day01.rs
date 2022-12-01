use super::text_file_to_vec;

pub fn main() {
    let raw_data = text_file_to_vec("data/day01.txt");

    let max_calories = find_max_calories(&raw_data);
    println!("day01 part1: {}", max_calories);

    let max_calories = find_calories_of_top_groups(&raw_data);
    println!("day01 part2 {}", max_calories);
}

fn get_elf_calories(raw_data: &Vec<String>) -> Vec<u32> {
    let mut elf_calories = Vec::<u32>::new();

    let mut curr_calories = 0;
    for line in raw_data {
        if line.len() == 0 {
            elf_calories.push(curr_calories);
            curr_calories = 0;
        } else {
            let snack_calories = line.parse::<u32>().unwrap();
            curr_calories += snack_calories;
        }
    }

    // Add the final line
    elf_calories.push(curr_calories);

    elf_calories
}

fn find_max_calories(raw_data: &Vec<String>) -> u32 {
    let elf_calories = get_elf_calories(raw_data);
    *elf_calories.iter().max().unwrap()
}

fn find_calories_of_top_groups(raw_data: &Vec<String>) -> u32 {
    let mut elf_calories = get_elf_calories(raw_data);
    elf_calories.sort();
    elf_calories.reverse();
    elf_calories[0] + elf_calories[1] + elf_calories[2]
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_test_input() -> Vec<String> {
        let mut test_vector = Vec::new();
        for i in 1..11 {
            test_vector.push((i * 1000).to_string());
            if i == 3 || i == 4 || i == 6 || i == 9 {
                test_vector.push(String::from(""));
            }
        }

        test_vector
    }

    #[test]
    fn test_find_elf_with_max_calories() {
        assert_eq!(find_max_calories(&get_test_input()), 24000);
    }

    #[test]
    fn test_find_calories_of_top_groups() {
        assert_eq!(find_calories_of_top_groups(&get_test_input()), 45000);
    }
}
