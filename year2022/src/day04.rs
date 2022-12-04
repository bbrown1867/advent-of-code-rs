use super::text_file_to_vec;
use std::str::FromStr;

pub fn main() {
    let raw_data = text_file_to_vec("data/day04.txt");

    let result = count_full_overlap(&raw_data);
    assert_eq!(result, 515);

    let result = count_partial_overlap(&raw_data);
    assert_eq!(result, 883);
}

struct Range {
    min: u32,
    max: u32,
}

impl FromStr for Range {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once("-") {
            Some((min, max)) => {
                match (u32::from_str(min), u32::from_str(max)) {
                    (Ok(min), Ok(max)) => Ok(Range { min, max }),
                    (_, _) => Err(()),
                }
            }
            None => Err(()),
        }
    }
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.min <= other.min && self.max >= other.max
    }

    fn overlaps(&self, other: &Range) -> bool {
        self.min <= other.max && self.max >= other.min
    }
}

fn count_full_overlap(raw_data: &Vec<String>) -> i32 {
    raw_data
        .into_iter()
        .map(|elf_pair| {
            let (left_elf, right_elf) = elf_pair.split_once(",").unwrap();
            let left_elf = Range::from_str(left_elf).unwrap();
            let right_elf = Range::from_str(right_elf).unwrap();
            (left_elf.contains(&right_elf) || right_elf.contains(&left_elf))
                as i32
        })
        .sum()
}

fn count_partial_overlap(raw_data: &Vec<String>) -> i32 {
    raw_data
        .into_iter()
        .map(|elf_pair| {
            let (left_elf, right_elf) = elf_pair.split_once(",").unwrap();
            let left_elf = Range::from_str(left_elf).unwrap();
            let right_elf = Range::from_str(right_elf).unwrap();
            left_elf.overlaps(&right_elf) as i32
        })
        .sum()
}
