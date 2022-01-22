use super::sub;
use super::text_file_to_vec;

pub fn main() {
    let raw_data = text_file_to_vec("data/day02.txt");

    let mut sub1 = sub::Sub::new(0, 0, 0, true);
    sub1.drive(&raw_data);
    println!("day02 part1: {}", sub1.get_product());

    let mut sub2 = sub::Sub::new(0, 0, 0, false);
    sub2.drive(&raw_data);
    println!("day02 part2: {}", sub2.get_product());
}
