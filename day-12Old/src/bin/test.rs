use std::fs;
use day_12::process_part1;


fn main() {
    let file = fs::read_to_string("./inputDemo.txt").unwrap();
    println!("{}", process_part1(&file));
}