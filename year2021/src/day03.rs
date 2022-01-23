use super::text_file_to_vec;

pub fn main() {
    let raw_data = text_file_to_vec("data/day03.txt");

    let num_bits = raw_data[0].len();
    let diagnostic_report = raw_data
        .iter()
        .map(|x| u32::from_str_radix(x, 2).unwrap())
        .collect();

    println!(
        "day03 part1: {}",
        get_power_consumption(&diagnostic_report, num_bits)
    );
    println!(
        "day03 part2: {}",
        get_life_support(&diagnostic_report, num_bits)
    );
}

fn get_power_consumption(diagnostic_report: &Vec<u32>, num_bits: usize) -> u32 {
    let (num_zeros, num_ones) =
        count_zeros_and_ones(&diagnostic_report, num_bits);

    // Compute gamma and epsilon
    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;
    let mut bitpos = 0;
    while bitpos < num_bits {
        if num_ones[bitpos] > num_zeros[bitpos] {
            gamma_rate |= 1 << bitpos;
        } else if num_ones[bitpos] < num_zeros[bitpos] {
            epsilon_rate |= 1 << bitpos;
        } else {
            panic!("Instructions unclear.")
        }

        bitpos += 1;
    }

    gamma_rate * epsilon_rate
}

fn get_life_support(diagnostic_report: &Vec<u32>, num_bits: usize) -> u32 {
    let mut report_for_oxygen = diagnostic_report.clone();
    let oxygen = search_and_filter(
        &mut report_for_oxygen,
        num_bits,
        num_bits as i32 - 1,
        true,
    );

    let mut report_for_co2 = diagnostic_report.clone();
    let co2 = search_and_filter(
        &mut report_for_co2,
        num_bits,
        num_bits as i32 - 1,
        false,
    );

    oxygen * co2
}

fn search_and_filter(
    diagnostic_report: &mut Vec<u32>,
    num_bits: usize,
    bitpos: i32,
    most_common: bool,
) -> u32 {
    if diagnostic_report.len() == 1 {
        diagnostic_report[0]
    } else {
        let (num_zeros, num_ones) =
            count_zeros_and_ones(diagnostic_report, num_bits);

        let num_zeros_for_curr_bit = num_zeros[bitpos as usize];
        let num_ones_for_curr_bit = num_ones[bitpos as usize];

        let filter_val = if num_zeros_for_curr_bit > num_ones_for_curr_bit {
            if most_common {
                0
            } else {
                1
            }
        } else {
            if most_common {
                1
            } else {
                0
            }
        };

        diagnostic_report
            .retain(|&x| (x & (1 << bitpos)) >> bitpos == filter_val);

        return search_and_filter(
            diagnostic_report,
            num_bits,
            bitpos - 1,
            most_common,
        );
    }
}

fn count_zeros_and_ones(
    diagnostic_report: &Vec<u32>,
    num_bits: usize,
) -> (Vec<u32>, Vec<u32>) {
    let mut count_zeros = Vec::new();
    let mut count_ones = Vec::new();
    let mut idx = 0;
    while idx < num_bits {
        count_zeros.push(0);
        count_ones.push(0);
        idx += 1;
    }

    for binary_number in diagnostic_report {
        let mut bitpos = 0;
        while bitpos < num_bits {
            match (binary_number & (1 << &bitpos)) > 0 {
                false => count_zeros[bitpos as usize] += 1,
                true => count_ones[bitpos as usize] += 1,
            }

            bitpos += 1;
        }
    }

    (count_zeros, count_ones)
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_test_input() -> Vec<u32> {
        vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111",
            "11100", "10000", "11001", "00010", "01010",
        ]
        .iter()
        .map(|x| u32::from_str_radix(x, 2).unwrap())
        .collect()
    }

    #[test]
    fn test_get_power_consumption() {
        assert_eq!(get_power_consumption(&get_test_input(), 5), 198);
    }

    #[test]
    fn test_get_life_support() {
        assert_eq!(get_life_support(&get_test_input(), 5), 230);
    }
}
