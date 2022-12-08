use adventofcode_2022::read_input;

pub fn prepare(file_name: &str) -> Vec<u16> {
    read_input(file_name)
        .lines()
        .map(|n| u16::from_str_radix(n,2).unwrap())
        .collect::<Vec<u16>>()
}

pub fn calculate_power_consumption(input: &[u16]) -> (u16, u16) {
    let half = input.len() / 2;
    let mut value_a:u16 = 0;
    let mut value_b:u16 = 0;

    for i in 0..16 {
        let mask:u16 = 1 << i;
        let bit_count = input
            .iter()
            .filter(|n| mask & **n > 0)
            .count();
        if bit_count > half {
            value_a += mask;
        } else if bit_count > 0 {
            value_b += mask;
        } else {
            break
        }
    }
    (value_a, value_b)
}

// pub fn calculate_oxygen(_input: &[u16]) -> Vec<u16> {
//     unimplemented!()
// }


pub fn part_1(input: &[u16]) -> Option<u32> {
    let (gamma_rate, epsilon_rate) = calculate_power_consumption(input);
    let oxygen_rate:u32 = gamma_rate as u32 * epsilon_rate as u32;
    Some(oxygen_rate)
}

// pub fn part_2(_input: &[u16]) -> Option<u16> {
//     unimplemented!()
// }

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_prepare() {
        let input = prepare("day03-example.txt");
        assert_eq!(input[0], 0b00100)
    }

    #[test]
    fn test_common_bits() {
        let input = prepare("day03-example.txt");
        let (value_a, value_b) = calculate_power_consumption(&input);
        assert_eq!(value_a, 22);
        assert_eq!(value_b, 9)
    }

    #[test]
    fn test_part_1() {
        let input = prepare("day03-example.txt");
        assert_eq!(part_1(&input), Some(198))
    }

    // #[test]
    // fn test_part_2() {
    //     let input = prepare("day03-example.txt");
    //     assert_eq!(part_2(&input), Some(230))
    // }

}
