use core::str::Chars;

pub enum CharType {
    Start,
    Middle,
    End
}
pub fn eval_neighbour(idx: usize, char_type: CharType, previous: &Option<Chars>, current: &Chars,  next: &Option<Chars>, ) {

}

pub fn process_part1(input: &str) -> u32 {
    let lines: Vec<Vec<char>>  = input
        .split("\n")
        .map(|line| line.chars().collect())
        .collect();
    println!("{:?}", lines);
    let mut previous_line: Option<&Vec<char>> = None;
    let mut next_line: Option<&Vec<char>> = Some(&lines[0]);
    let line_length = lines[0].len();
    let line_sum = 0;
    for i in 0..lines.len() {
        println!("{}",i);
        println!("{:?}", previous_line);
        println!("{:?}", lines[i]);
        println!("{:?}", next_line);


        if i >= lines.len() - 2 {
            next_line = None;
        } else {
            next_line = Some(&lines[i+2]);
        }
        previous_line = Some(&lines[i]);
        println!("----------");
    }
    32
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = process_part1(&input);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_second() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = process_part2(&input);
        assert_eq!(result, 2286);
    }

}