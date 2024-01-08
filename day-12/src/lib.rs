use std::collections::HashMap;

pub fn calculate(previous: char, i: usize, j: usize, current_damaged: usize, springs: &Vec<char>, damaged: &Vec<usize>, cache: &mut HashMap<(char, usize, usize, usize), i64> ) -> i64 {

    // Get recorded cache value
    if i < springs.len() {
        if let Some(val) = cache.get(&(previous, current_damaged,i,j)) {
            return *val;
        }
    }

    // Too many contiguous groups
    if j == damaged.len() {
        return 0;
    }

    // If end of the line
    if i == springs.len() {
        // Too few contiguous groups
        if j < damaged.len() - 1 {
            return 0;
        }
        // check last contiguous group
        return match current_damaged == damaged[j] {
            true => 1,
            false => 0
            }
    }

    let out = match springs[i] {
        '.' => {
                match previous {
                    '#' => {
                        // End of contiguous group, check if len ok
                        if current_damaged != damaged[j] {
                            return 0;
                        } else {
                            return calculate('.', i+1, j, current_damaged, springs, damaged, cache)   ;
                        }
                    },
                    _ => {
                        return calculate('.', i+1, j, current_damaged, springs, damaged, cache) ;
                    }
                };
            }
        '#' =>  {
            match previous {
                '.' => {
                    calculate('#', i + 1, j + 1, 1, springs, damaged, cache)
                    }
                '#' => {
                    calculate('#', i+1, j, current_damaged+1, springs, damaged, cache)
                }
                _ => unreachable!(),
            }
        }
        '?' =>  {
            // if current = '?', we can replace it with either a '.' or a '#'
            match previous {
                '.' => {
                    let res_dot = calculate('.', i + 1, j, current_damaged, springs, damaged, cache);
                    let res_hash = calculate('#', i + 1, j + 1, 1, springs, damaged, cache);
                    res_dot + res_hash
                }
                '#' => {
                    let res_dot = {
                        // End of contiguous group, check if len ok
                        if j == damaged.len() {
                            return 0;
                        }
                        if current_damaged != damaged[j] {
                            0
                        } else {
                            calculate('.', i + 1, j, current_damaged, springs, damaged, cache)
                        }
                    };
                    let res_hash = {
                        calculate('#', i + 1, j, current_damaged + 1, springs, damaged, cache)
                    };
                    res_dot + res_hash
                },
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    };
    // Insert into cache
    cache.insert((previous, current_damaged,i,j), out);
    out
}



pub fn process_part1(input: &str) -> i64 {
    let out = input
        .split("\n")
        .map(|line| {
            let (springs_str, damaged_str) = line.split_once(' ').unwrap();
            let springs = springs_str.chars().collect::<Vec<char>>();
            let mut damaged = vec![0];
            damaged.extend(damaged_str.split(',').map(|nr| nr.parse().unwrap()).collect::<Vec<usize>>());
            let mut cache: HashMap<(char, usize, usize, usize), i64> = HashMap::new();
            let out = calculate('.', 0, 0, 0, &springs, &damaged, &mut cache );
            out
        }).sum();
    out
}

pub fn process_part2(input: &str) -> i64 {
    let out = input
        .split("\n")
        .map(|line| {
            let (springs_str, damaged_str) = line.split_once(' ').unwrap();
            let springs_single = springs_str.chars().collect::<Vec<char>>();
            let damaged_single = damaged_str.split(',').map(|nr| nr.parse().unwrap()).collect::<Vec<usize>>();
            let mut springs= vec![];
            let mut damaged = vec![0];
            for _ in 0..4 {
                springs.extend(springs_single.clone());
                springs.push('?');
                damaged.extend(damaged_single.clone());
            }
            springs.extend(springs_single.iter().cloned());
            damaged.extend(damaged_single.clone());
            let mut cache: HashMap<(char, usize, usize, usize), i64> = HashMap::new();
            let out = calculate('.', 0, 0, 0, &springs, &damaged, &mut cache );
            out
        }).sum();
    out
}

////////////////////////////////////////


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_first() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        let result = process_part1(&input);
        assert_eq!(result, 21);
    }

    #[test]
    fn test_second() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        let result = process_part2(&input);
        assert_eq!(result, 525152);
    }

}