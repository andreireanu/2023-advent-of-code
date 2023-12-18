pub fn process_part1(input: &str) -> i32 {
    input
        .split("\n")
        .map(|line|
            {
                let (_, right) = line.split_once(": ").unwrap();
                let (left, right) = right.split_once(" | ").unwrap();
                let left = left
                    .split_whitespace()
                    .map(|element| {
                        element.parse::<u32>().unwrap()
                    })
                    .collect::<Vec<u32>>();
                let right = right
                    .split_whitespace()
                    .map(|element| {
                        element.parse::<u32>().unwrap()
                    })
                    .collect::<Vec<u32>>();
                let intersection: Vec<_> = left.into_iter().filter(|&x| right.contains(&x)).collect();
                let win_count = intersection.len();
                if win_count == 0 {
                    return 0;
                } else {
                    return 2_i32.pow((win_count - 1) as u32);
                }
            })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let result = process_part1(&input);
        assert_eq!(result, 13);
    }

}