use adventofcode_2022::read_input;
use itertools::Itertools;

#[derive(Debug, Eq, PartialEq)]
pub enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
    None
}

struct Location {
    position: i32,
    depth: i32,
    aim: i32
}

pub fn prepare(file_name: &str) -> Vec<Command> {
    let mut input = Vec::<Command>::new();
    for line in read_input(file_name).lines() {
        let parts:(&str, &str) = line.split(' ').next_tuple().unwrap();
        input.push(match parts {
            ("forward", val) => Command::Forward(val.parse::<i32>().unwrap()),
            ("down", val) => Command::Down(val.parse::<i32>().unwrap()),
            ("up", val) => Command::Up(val.parse::<i32>().unwrap()),
            (&_, _) => Command::None
        });
    }
    input
}

pub fn part_1(commands: &Vec<Command>) -> Option<i32> {
    let mut pos = Location{position: 0, depth: 0, aim: 0};
    for cmd in commands {
        match cmd {
            Command::Forward(n) => pos.position += n,
            Command::Down(n) => pos.depth += n,
            Command::Up(n) => pos.depth -= n,
            Command::None => ()
        };
    }
    Some(pos.position * pos.depth)
}

pub fn part_2(commands: &Vec<Command>) -> Option<i32> {
    let mut pos = Location{position: 0, depth: 0, aim: 0};
    for cmd in commands {
        match cmd {
            Command::Forward(n) => {
                pos.position += n;
                pos.depth += n * pos.aim;
            },
            Command::Down(n) => pos.aim += n,
            Command::Up(n) => pos.aim -= n,
            Command::None => ()
        };
    }
    Some(pos.position * pos.depth)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_prepare() {
        let input = prepare("day02-example.txt");
        assert_eq!(input[0], Command::Forward(5))
    }

    #[test]
    fn test_part_1() {
        let input = prepare("day02-example.txt");
        assert_eq!(part_1(&input), Some(150))
    }

    #[test]
    fn test_part_2() {
        let input = prepare("day02-example.txt");
        assert_eq!(part_2(&input), Some(900))
    }

}