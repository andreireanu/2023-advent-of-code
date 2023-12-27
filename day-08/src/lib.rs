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

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}

pub fn process_part2(input: &str) -> u64 {

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
    let mut counts = Vec::new();
    let mut count;
    for node in hm.keys() {
        if (*node).chars().nth(2).unwrap() == 'A' {
            count = 0;
            let mut current = *node;
            'outer: loop {
                for c in nodes.chars() {
                    count += 1;
                    let left_right = hm.get(&current).unwrap();
                    if c == 'L'{
                        current = left_right.0;
                    } else {
                        current = left_right.1;
                    }
                    if current.chars().nth(2).unwrap() == 'Z' {
                        break 'outer;
                    }
                }
            }
            counts.push(count);
        }
    }
    counts.iter().fold(counts[0], |acc, &x| lcm(acc, x))
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

    #[test]
    fn test_second() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        let result = process_part2(&input);
        assert_eq!(result, 6);
    }



}