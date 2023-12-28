pub fn calculate_next(mut v: Vec<i64>) -> i64 {
    let len = v.len();
    for j in 1..len {
        for i in 0..len - j {
            v[i] = v[i+1] - v[i];
        }
    }
    v.iter().sum::<i64>()
}

pub fn process_part1(input: &str) -> i64 {
    input
        .split("\n")
        .map(|line| {
            let v = line
                .split(' ')
                .map(|value| value.parse().unwrap())
                .collect::<Vec<i64>>();
            calculate_next(v)
        }).sum()
}

pub fn process_part2(input: &str) -> i64 {
    input
        .split("\n")
        .map(|line| {
            let mut v = line
                .split(' ')
                .map(|value| value.parse().unwrap())
                .collect::<Vec<i64>>();
            v.reverse();
            calculate_next(v)
        }).sum()
}

////////////////////////////////////////


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_first() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let result = process_part1(&input);
        assert_eq!(result, 114);
    }



}