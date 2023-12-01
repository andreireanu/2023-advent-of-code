use core::str::Chars;

pub fn get_first_char(mut line_chars: Vec<char>) -> char {
    let mut iter = line_chars.iter();
    while let Some(c) = iter.next() {
        if c.is_numeric() {
            return *c;
        }
    }
    ' '
}

pub fn process_part1(input: &String) -> u32 {
    input
        .split("\n")
        .map(|line|
              {
                  let reversed_chars: Vec<char> = line.chars().rev().collect();
                  let chars: Vec<char> = line.chars().collect();
                  let first = get_first_char(chars);
                  let second = get_first_char(reversed_chars);
                  let calibration_value = first.to_digit(10).unwrap() * 10 + second.to_digit(10).unwrap();
                  calibration_value
              })
        .sum()
}

