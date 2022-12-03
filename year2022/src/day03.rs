use super::text_file_to_vec;

pub fn main() {
    let raw_data = text_file_to_vec("data/day03.txt");

    let result = find_sum_of_duplicate_items(&raw_data);
    assert_eq!(result, 7742);

    let result = find_sum_of_badge_groups(&raw_data);
    assert_eq!(result, 2276);
}

fn find_sum_of_duplicate_items(raw_data: &Vec<String>) -> u32 {
    let mut sum = 0;
    for (c1, c2) in parse_rucksack_compartments(&raw_data) {
        let dups = find_duplicate_items(&c1, &c2);
        assert!(dups.len() <= 1);
        sum += dups.iter().sum::<u32>();
    }

    sum
}

fn find_sum_of_badge_groups(raw_data: &Vec<String>) -> u32 {
    let rucksacks = parse_rucksacks(&raw_data);

    let mut sum = 0;
    let mut index = 0;
    while index < rucksacks.len() {
        let elf1 = &rucksacks[index];
        let elf2 = &rucksacks[index + 1];
        let elf3 = &rucksacks[index + 2];
        sum += find_common_item(&elf1, &elf2, &elf3);
        index += 3;
    }

    sum
}

fn find_common_item(
    rucksack1: &Vec<u32>,
    rucksack2: &Vec<u32>,
    rucksack3: &Vec<u32>,
) -> u32 {
    let dup_set1 = find_duplicate_items(rucksack1, rucksack2);
    let dup_set2 = find_duplicate_items(&dup_set1, rucksack3);
    assert_eq!(dup_set2.len(), 1);
    dup_set2[0]
}

fn find_duplicate_items(
    compartment1: &Vec<u32>,
    compartment2: &Vec<u32>,
) -> Vec<u32> {
    let mut dups = Vec::new();
    for item1 in compartment1 {
        match compartment2.into_iter().find(|&item2| item2 == item1) {
            Some(duplicate) => {
                // Avoid duplicates in the result
                match dups.iter().find(|&&dup| dup == *duplicate) {
                    Some(_) => (),
                    None => dups.push(*duplicate),
                };
            }
            None => continue,
        }
    }

    dups
}

fn item_to_priority(item: char) -> u32 {
    let converted = item as u32;
    // Convert from ASCII table values to priorit
    if 97 <= converted && converted <= 122 {
        converted - 96
    } else {
        converted - 64 + 26
    }
}

fn parse_rucksacks(raw_data: &Vec<String>) -> Vec<Vec<u32>> {
    raw_data
        .iter()
        .map(|rucksack| rucksack.chars().map(|i| item_to_priority(i)).collect())
        .collect()
}

fn parse_rucksack_compartments(
    raw_data: &Vec<String>,
) -> Vec<(Vec<u32>, Vec<u32>)> {
    raw_data
        .iter()
        .map(|rucksack| {
            let index = rucksack.len() / 2;
            let compartments = rucksack.split_at(index);
            let compartment1 = compartments
                .0
                .chars()
                .map(|i| item_to_priority(i))
                .collect();
            let compartment2 = compartments
                .1
                .chars()
                .map(|i| item_to_priority(i))
                .collect();
            (compartment1, compartment2)
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_test_input() -> Vec<String> {
        vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect()
    }

    #[test]
    fn test_find_sum_of_duplicate_items() {
        assert_eq!(find_sum_of_duplicate_items(&get_test_input()), 157);
    }

    #[test]
    fn test_find_sum_of_badge_groups() {
        assert_eq!(find_sum_of_badge_groups(&get_test_input()), 70);
    }
}
