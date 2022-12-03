use super::text_file_to_vec;

pub fn main() {
    let raw_data = text_file_to_vec("data/dayX.txt");

    let result = 0;
    println!("dayX part1: {}", result);

    let result = 0;
    println!("dayX part2: {}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_test_input() -> Vec<String> {
        vec![""].iter().map(|x| x.to_string()).collect()
    }

    #[test]
    fn test() {}
}
