use std::collections::HashMap;

pub fn process_part1(input: &str) -> i64 {

    let (nodes, map) = input.split_once("\n\n").unwrap();
    let mut hm = HashMap::new();
    map
        .split("\n")
        .for_each(|line| {
            let (node, left_right) = line.split_once(" = ").unwrap();
            let left = &left_right[1..4];
            let right = &left_right[6..9];
            hm.insert(node, (left,right));
        });
    let mut count = 0;
    let mut current = "AAA";
    while current != "ZZZ" {
        for c in nodes.chars() {
            count += 1;
            let left_right = hm.get(&current).unwrap();
            if c == 'L'{
                current = left_right.0;
            } else {
                current = left_right.1;
            }
        }
    }
    count
}

////////////////////////////////////////




#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_first() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        let result = process_part1(&input);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_first_2nd_example() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let result = process_part1(&input);
        assert_eq!(result, 6);
    }

}