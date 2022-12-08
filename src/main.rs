
pub mod day01;
pub mod day02;
pub mod day03;
pub mod day24;

fn main() {
    println!("=== Advent of Code 2021 ====");

    println!("---------- Day 01 ----------");
    let input_1 = day01::prepare("day01.txt");
    println!("⭐ {}", day01::part_1(&input_1).unwrap());
    println!("⭐ {}", day01::part_2(&input_1).unwrap());

    println!("---------- Day 02 ----------");
    let input_2 = day02::prepare("day02.txt");
    println!("⭐ {}", day02::part_1(&input_2).unwrap());
    println!("⭐ {}", day02::part_2(&input_2).unwrap());

    println!("---------- Day 03 ----------");
    let input_3 = day03::prepare("day03.txt");
    println!("⭐ {}", day03::part_1(&input_3).unwrap());
    // println!("⭐ {}", day03::part_2(&input_3).unwrap());

    println!("---------- Day 24 ----------");
    println!("⭐ {}", day24::part_1().unwrap());
    println!("⭐ {}", day24::part_2().unwrap());
}
