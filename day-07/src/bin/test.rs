use std::fs;
use day_07::process_part1;
use day_07::process_part2;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("{}", process_part1(&file));
    println!("{}", process_part2(&file));
}