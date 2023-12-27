use std::collections::HashMap;

pub fn process_part1(input: &str) -> i64 {

    input
        .split("\n")
        .for_each(|line| {
            println!("{}", line);
        });

    42
}

////////////////////////////////////////




#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_first() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let result = process_part1(&input);
        assert_eq!(result, 6440);
    }

    fn test_second() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let result = process_part2(&input);
        assert_eq!(result, 5905);
    }

}