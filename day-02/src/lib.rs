pub fn process_part1(input: &str) -> u32 {
    input
        .split("\n")
        .map(|line| {
            let (game, game_data) = line.split_once(": ").unwrap();
            let (_, game_number_string) = game.split_once(" ").unwrap();
            let mut game_number = game_number_string.parse::<u32>().unwrap();
            let sets: Vec<&str> = game_data.split("; ").collect();
            'outer: for set in sets {
                let draw: Vec<&str> = set.split(", ").collect();
                for pick in draw {
                    let (number_string, color) = pick.split_once(" ").unwrap();
                    let number = number_string.parse::<u32>().unwrap();
                    match color {
                        "red" => {
                            if number > 12 {
                                game_number = 0;
                                break 'outer
                        }}
                        "green" => {
                            if number > 13 {
                                game_number = 0;
                                break 'outer
                            }}
                        "blue" => {
                            if number > 14 {
                                game_number = 0;
                                break 'outer
                            }}
                        _ => unreachable!(),
                    }
                }
            }
            game_number
        }
        )
        .sum()
}

pub fn process_part2(input: &str) -> u32 {
    input
        .split("\n")
        .map(|line| {
            let mut cubes = [0, 0, 0];
            let (_, game_data) = line.split_once(": ").unwrap();
            let sets: Vec<&str> = game_data.split("; ").collect();
            for set in sets {
                let draw: Vec<&str> = set.split(", ").collect();
                for pick in draw {
                    let (number_string, color) = pick.split_once(" ").unwrap();
                    let number = number_string.parse::<u32>().unwrap();
                    match color {
                        "red" => cubes[0] = number.max(cubes[0]),
                        "green" => cubes[1] = number.max(cubes[1]),
                        "blue" => cubes[2] = number.max(cubes[2]),
                        _ => unreachable!(),
                    }
                }
            }
            cubes[0] * cubes[1] * cubes[2]
        }
        )
        .sum()
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