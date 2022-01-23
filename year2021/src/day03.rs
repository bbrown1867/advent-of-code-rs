use super::text_file_to_vec;

pub fn main() {
    let raw_data = text_file_to_vec("data/day03.txt");

    println!("day03 part1: {}", get_power_consumption(&raw_data));
}

fn get_power_consumption(diagnostic_report: &Vec<String>) -> u32 {
    let num_bits = diagnostic_report[0].len();

    // Create vectors, initialize to 0
    let mut count_zeros = Vec::new();
    let mut count_ones = Vec::new();
    let mut idx = 0;
    while idx < num_bits {
        count_zeros.push(0);
        count_ones.push(0);
        idx += 1;
    }

    // Count the number of zeros and ones for each bit position
    for binary_number_as_str in diagnostic_report {
        let binary_number: u32 =
            u32::from_str_radix(binary_number_as_str, 2).unwrap();

        let mut bitpos = 0;
        while bitpos < num_bits {
            match (binary_number & (1 << &bitpos)) > 0 {
                false => count_zeros[bitpos as usize] += 1,
                true => count_ones[bitpos as usize] += 1,
            }

            bitpos += 1;
        }
    }

    // Compute gamma and epsilon
    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;
    let mut bitpos = 0;
    while bitpos < num_bits {
        if count_ones[bitpos] > count_zeros[bitpos] {
            gamma_rate |= 1 << bitpos;
        } else if count_ones[bitpos] < count_zeros[bitpos] {
            epsilon_rate |= 1 << bitpos;
        } else {
            panic!("Instructions unclear.")
        }

        bitpos += 1;
    }

    gamma_rate * epsilon_rate
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_test_input() -> Vec<String> {
        vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111",
            "11100", "10000", "11001", "00010", "01010",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect()
    }

    #[test]
    fn test_get_power_consumption() {
        assert_eq!(get_power_consumption(&get_test_input()), 198);
    }
}
