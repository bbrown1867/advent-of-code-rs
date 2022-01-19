use super::text_file_to_vec;

pub fn main() {
    let raw_data = text_file_to_vec("data/day01.txt");
    let parsed_data: Vec<u32> =
        raw_data.iter().map(|x| x.parse::<u32>().unwrap()).collect();

    let result = count_depth_increases(&parsed_data);
    println!("day01 part1: {}", result);

    let result = count_depth_increases_window(&parsed_data, 3);
    println!("day01 part2: {}", result);
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

fn count_depth_increases_window(
    sonar_sweep: &Vec<u32>,
    window_size: usize,
) -> u32 {
    assert!(window_size <= sonar_sweep.len());

    let mut num_increases: u32 = 0;
    let mut window: Vec<u32> = Vec::new();

    for data_point in sonar_sweep {
        if window.len() < window_size {
            window.push(data_point.clone());
        } else {
            let curr_window_sum: u32 = window.iter().sum();

            for i in 0..(window_size - 1) {
                window[i] = window[i + 1];
            }
            window[window_size - 1] = data_point.clone();

            let new_window_sum: u32 = window.iter().sum();

            if new_window_sum > curr_window_sum {
                num_increases += 1;
            }
        }
    }

    num_increases
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_test_input() -> Vec<u32> {
        vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]
    }

    #[test]
    fn test_count_depth_increases() {
        assert_eq!(count_depth_increases(&get_test_input()), 7);
    }

    #[test]
    fn test_count_depth_increases_window() {
        assert_eq!(count_depth_increases_window(&get_test_input(), 3), 5);
    }
}
