use core::cmp::max;
use core::cmp::min;
use std::collections::HashMap;

pub fn eval_neighbours_part1(line_length: usize, start_idx: usize, end_idx: usize, previous: Option<&Vec<char>>, current: &Vec<char>, next:  Option<&Vec<char>>) -> bool {

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
    let mut is_valid;
    let mut sum = 0;
    for i in 0..lines.len() {
        let mut start_idx = 0;
        let mut end_idx;
        let mut is_digit = false;
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
                        is_valid = eval_neighbours_part1(line_length, start_idx, end_idx, previous_line, &lines[i], next_line );
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
                        is_valid = eval_neighbours_part1(line_length, start_idx, end_idx, previous_line, &lines[i], next_line );
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



pub fn create_key(row: usize, column: isize) -> String {
    format!("{}-{}", row, column)
}

pub fn eval_neighbours_part2(line_length: usize, start_idx: usize, end_idx: usize, previous: Option<&Vec<char>>, current: &Vec<char>, next: Option<&Vec<char>>, gears_map: &mut HashMap<String, Vec<i32>>, current_num: u32, row: usize )   {

    let eval_start: isize = max(0, start_idx as isize -1);
    let eval_end: isize = min(end_idx as isize,line_length as isize);

    if let Some(previous_line) = previous {
        for i in eval_start..=eval_end {
            match previous_line[i as usize] {
                '*'  => {
                    let key = create_key(row - 1,i);
                    let data_vec = gears_map.entry(key).or_insert(vec![]);
                    data_vec.push(current_num as i32);
                }
                _ => { continue; }
            }
        }
    }

    match current[eval_start as usize] {
        '*'  => {
            let key = create_key(row,eval_start);
            let data_vec = gears_map.entry(key).or_insert(vec![]);
            data_vec.push(current_num as i32);
        }
        _ => { }
    }

    match current[eval_end as usize] {
        '*'  => {
            let key = create_key(row,eval_end);
            let data_vec = gears_map.entry(key).or_insert(vec![]);
            data_vec.push(current_num as i32);
        }
        _ => {  }
    }

    if let Some(next_line) = next {
        for i in eval_start..=eval_end {
            match next_line[i as usize] {
                '*'  => {
                    let key = create_key(row + 1,i);
                    let data_vec = gears_map.entry(key).or_insert(vec![]);
                    data_vec.push(current_num as i32);
                }
                _ => { continue; }
            }
        }
    }
}

pub fn process_part2(input: &str) -> u32 {
    let lines: Vec<Vec<char>>  = input
        .split("\n")
        .map(|line| line.chars().collect())
        .collect();
    let mut previous_line: Option<&Vec<char>> = None;
    let mut next_line: Option<&Vec<char>> = Some(&lines[1]);
    let line_length= lines[0].len();
    let mut sum = 0;
    let mut gears_map: HashMap<String, Vec<i32>> = HashMap::new();
    for i in 0..lines.len() {
        let mut start_idx = 0;
        let mut end_idx;
        let mut is_digit = false;
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
                        eval_neighbours_part2(line_length, start_idx, end_idx, previous_line, &lines[i], next_line, &mut gears_map, current_num, i  );
                        current_num = 0;
                    }

                }
                _ => {
                    if is_digit == true {
                        is_digit = false;
                        end_idx = idx;
                        eval_neighbours_part2(line_length, start_idx, end_idx, previous_line, &lines[i], next_line, &mut gears_map, current_num, i );
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
    for (_,v) in gears_map {
        if v.len() == 2 {
            sum += (v[0] * v[1]) as u32;
        }
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

    #[test]
    fn test_second() {
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
        let result = process_part2(&input);
        assert_eq!(result, 467835);
    }

}