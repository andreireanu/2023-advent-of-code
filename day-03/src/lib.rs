use core::str::Chars;

pub enum CharType {
    Start,
    Middle,
    End
}
pub fn eval_neighbour(idx: usize, char_type: CharType, previous: &Option<Chars>, current: &Chars,  next: &Option<Chars>, ) {

}

pub fn process_part1(input: &str) -> u32 {
    let mut previous_line: &str = "";
    let mut next_line: &str = "";
    let lines = input
        .split("\n")
        .collect::<Vec<&str>>();
    let mut previous_line: Option<Chars> = None;
    let mut next_line: Option<Chars> = Some(lines[1].chars());
    let line_length = lines[0].len();
    let line_sum = 0;
    for i in 0..lines.len() {
        println!("{}",i);
        println!("{:?}", previous_line);
        let current_line = lines[i].chars();
        println!("{:?}", current_line);
        println!("{:?}", next_line);


        if i >= lines.len() - 2 {
            next_line = None;
        } else {
            next_line = Some(lines[i+2].chars());
        }
        previous_line = Some(current_line);
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