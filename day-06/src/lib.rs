pub fn process_line_part1(line: &str) -> Vec<i64> {
    line.split_whitespace()
        .map(|el| el.to_string())
        .filter_map(|el| el.parse().ok())
        .collect::<Vec<i64>>()
}

pub fn process_line_part2(line: &str) -> i64 {
    line
        .split_whitespace()
        .fold( "".to_string(), |mut acc, el| {
            acc.push_str(el);
            acc
        })
        .parse::<i64>().unwrap()
}


pub fn process_part1(input: &str) -> i64 {
    let lines = input.split('\n').collect::<Vec<&str>>();
    let times = process_line_part1(lines[0]);
    let durations = process_line_part1(lines[1]);
    let zipped: Vec<(&i64, &i64)> = times.iter().zip(durations.iter()).collect();
    let mut multiplied = 1;
    for (time, duration) in zipped{
        let mut count = 0;
        for hold in 1..*time {
            if hold * (*time - hold) > *duration {
                count += 1;
            }
        }
        multiplied *= count;
    }
    multiplied
}

pub fn process_part2(input: &str) -> i64 {
    let lines = input.split('\n').collect::<Vec<&str>>();
    let (_, time) = lines[0].split_once(':').unwrap();
    let time = process_line_part2(time);
    let (_, duration) = lines[1].split_once(':').unwrap();
    let duration = process_line_part2(duration);
    let mut count = 0;
    for hold in 1..time {
        if hold * (time - hold) > duration {
            count += 1;
        }
    }
    count
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_first() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let result = process_part1(&input);
        assert_eq!(result, 288);
    }

    fn test_second() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let result = process_part2(&input);
        assert_eq!(result, 28360140);
    }
}