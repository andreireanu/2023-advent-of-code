use core::cmp::max;
use core::cmp::min;

pub fn eval_neighbours(line_length: usize, start_idx: usize, end_idx: usize, previous: Option<&Vec<char>>, current: &Vec<char>,  next:  Option<&Vec<char>>) -> bool {

    let eval_start: isize = max(0, start_idx as isize -1);
    let eval_end: isize = min(end_idx as isize,line_length as isize);

    if let Some(previous_line) = previous {
        for i in eval_start..=eval_end {
            match previous_line[i as usize] {
                '.' | '0'..='9' => { continue; }
                _ => { return true}
            }
        }
    }

    match current[eval_start as usize] {
        '.' | '0'..='9' => { }
        _ => { return true}
    }

    match current[eval_end as usize] {
        '.' | '0'..='9' => { }
        _ => { return true}
    }

    if let Some(next_line) = next {
        for i in eval_start..=eval_end {
            match next_line[i as usize] {
                '.' | '0'..='9' => { continue; }
                _ => { return true}
            }
        }
    }

    false
}

pub fn process_part1(input: &str) -> u32 {
    let lines: Vec<Vec<char>>  = input
        .split("\n")
        .map(|line| line.chars().collect())
        .collect();
    let mut previous_line: Option<&Vec<char>> = None;
    let mut next_line: Option<&Vec<char>> = Some(&lines[1]);
    let line_length= lines[0].len();
    let mut sum = 0;
    for i in 0..lines.len() {
        let mut start_idx = 0;
        let mut end_idx;
        let mut is_digit = false;
        let mut is_valid;
        let mut current_num = 0;
        for idx in 0..line_length  {
            match lines[i][idx]  {
                '0'..='9' => {
                    current_num = current_num * 10 + lines[i][idx].to_digit(10).unwrap();
                    if is_digit == false {
                        is_digit = true;
                        start_idx = idx;
                    };
                    if idx == line_length - 1 {
                        end_idx = idx;
                        is_valid = eval_neighbours(line_length, start_idx, end_idx, previous_line, &lines[i], next_line );
                        if is_valid {
                            sum += current_num;
                        }
                        current_num = 0;
                    }

                }
                _ => {
                    if is_digit == true {
                        is_digit = false;
                        end_idx = idx;
                        is_valid = eval_neighbours(line_length, start_idx, end_idx, previous_line, &lines[i], next_line );
                        if is_valid {
                            sum += current_num;
                        }
                        current_num = 0;
                    };
                }
            }
        }

        if i >= lines.len() - 2 {
            next_line = None;
        } else {
            next_line = Some(&lines[i+2]);
        }
        previous_line = Some(&lines[i]);
    }
    sum
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let result = process_part1(&input);
        assert_eq!(result, 4361);
    }

}