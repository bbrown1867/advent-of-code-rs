use super::text_file_to_vec;

pub fn day01_part1() {
    let raw_data = text_file_to_vec("data/day01.txt");
    let parsed_data: Vec<u32> =
        raw_data.iter().map(|x| x.parse::<u32>().unwrap()).collect();
    let result = count_depth_increases(&parsed_data);
    println!("day01_part1: {}", result);
}

fn count_depth_increases(sonar_sweep: &Vec<u32>) -> u32 {
    let mut num_increases: u32 = 0;
    let mut prev_data_point: Option<u32> = None;

    for data_point in sonar_sweep {
        match prev_data_point {
            Some(value) => {
                if data_point > &value {
                    num_increases += 1;
                }
            }
            None => (),
        }

        prev_data_point = Some(data_point.clone());
    }

    num_increases
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_depth_increases() {
        let test_input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(count_depth_increases(&test_input), 7);
    }
}
