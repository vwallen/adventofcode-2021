
use adventofcode_2022::read_input;

pub fn prepare(file_name: &str) -> Vec<u32> {
    read_input(file_name)
        .lines()
        .map(|n| n.parse::<u32>().unwrap())
        .collect()
}

pub fn part_1(input: &[u32]) -> Option<u32> {
    let output = input
        .windows(2)
        .filter(|sample| sample[1] > sample[0])
        .count() as u32;
    Some(output)
}

pub fn part_2(input: &[u32]) -> Option<u32> {
    let output = input
        .windows(3)
        .map(|sample| sample.iter().sum())
        .collect::<Vec<u32>>()
        .windows(2)
        .filter(|sample| sample[1] > sample[0])
        .count() as u32;
    Some(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prepare_elves() {
        let input = prepare("day01-example.txt");
        assert_eq!(input[0], 199);
        assert_eq!(input[input.len() - 1], 263)
    }

    #[test]
    fn test_part_1() {
        let input = prepare("day01-example.txt");
        assert_eq!(part_1(&input), Some(7))
    }

    #[test]
    fn test_part_2() {
        let input = prepare("day01-example.txt");
        assert_eq!(part_2(&input), Some(5))
    }
}
