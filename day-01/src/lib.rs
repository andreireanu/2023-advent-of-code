pub fn get_first_char(line_chars: Vec<char>) -> char {
    let mut iter = line_chars.iter();
    while let Some(c) = iter.next() {
        if c.is_numeric() {
            return *c;
        }
    }
    ' '
}

pub fn get_digit(input: &str, reverse: bool) -> u32 {
    let arr = ["one", "1", "two", "2", "three", "3", "four", "4", "five", "5", "six", "6", "seven", "7", "eight", "8", "nine", "9"];
    let mut out : String = String::new();
    if reverse == false {
        let mut ref_idx = usize::MAX;
        for el in arr.iter() {
            match input.find(el) {
                Some(idx) => {
                    if idx < ref_idx {
                        ref_idx = idx;
                        out = el.to_string();
                    }
                },
                None => {}
            }
        }
    } else {
        let mut ref_idx = 0;
        for el in arr.iter() {
            match input.rfind(el) {
                Some(idx) => {
                    if idx >= ref_idx {
                        ref_idx = idx;
                        out = el.to_string();
                    }
                },
                None => {}
            }
        }
    }
    match out.as_str() {
        "one"  => 1,
        "1" => 1,
        "two" => 2,
        "2" => 2,
        "three" => 3,
        "3" => 3,
        "four" => 4,
        "4" => 4,
        "five" => 5,
        "5" => 5,
        "six" => 6,
        "6" => 6,
        "seven" => 7,
        "7" => 7,
        "eight" => 8,
        "8" => 8,
        "nine" => 9,
        "9" => 9,
        _ => 0,
    }
}

pub fn process_part1(input: &str) -> u32 {
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

pub fn process_part2(input: &str) -> u32 {
    input
        .split("\n")
        .map(|line|
            {
                let first_digit = get_digit(line, false);
                let last_digit = get_digit(line, true);
                let out = first_digit * 10 + last_digit;
                out
            })
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let result = process_part1(&input);
        assert_eq!(result, 142);
    }
    #[test]
    fn test_second() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let result = process_part2(&input);
        assert_eq!(result, 281);
    }
}

