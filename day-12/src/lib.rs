use std::collections::HashMap;

pub fn calculate(previous: char, i: usize, j: usize, current_damaged: usize, springs: &Vec<char>, damaged: &Vec<usize>, cache: &mut HashMap<(char, usize, usize), i32> ) -> i32 {

    if let Some(val) = cache.get(&(previous,i,j)) {
        println!("Found {:?}: {}", (previous,i,j), *val);
        return *val;
    }

    if j == damaged.len() {
        return 0;
    }

    if i == springs.len() {
        if j < damaged.len() - 1 {
            return 0;
        }
        println!("{} {} {} {}", i, j, current_damaged, damaged[j]);
        return match current_damaged == damaged[j] {
            true => {
                println!("OK END");
                1}
            false => {
                println!("FALSE END");
                0
                }
            }
    }


    let out = match springs[i] {
        '.' => {
                match previous {
                    '#' => {
                        if current_damaged != damaged[j] {
                            println!("FAIL #.");
                            println!("{} {} {} {}", i, j, current_damaged, damaged[j]);
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
            match previous {
                '.' => {
                    let res_dot = {
                        println!("HERE1");
                        calculate('.', i + 1, j, current_damaged, springs, damaged, cache)
                    };
                    let res_hash = {
                        println!("HERE2");
                        calculate('#', i + 1, j + 1, 1, springs, damaged, cache)
                        };
                    res_dot + res_hash
                }
                '#' => {
                    let res_dot = {
                        println!("HERE3");
                        if j == damaged.len() {
                            println!("PASSED LEN");
                            return 0;
                        }
                        println!("{} {} {} {}", i, j, current_damaged, damaged[j]);

                        if current_damaged != damaged[j] {
                            println!("FAIL .#");
                            println!("{} {} {} {}", i, j, current_damaged, damaged[j]);
                            0
                        } else {
                            println!("Continue");
                            calculate('.', i + 1, j, current_damaged, springs, damaged, cache)
                        }
                    };
                    let res_hash = {
                        println!("HERE4");
                        println!("{} {} {} {}", i, j, current_damaged, damaged[j]);
                        calculate('#', i + 1, j, current_damaged + 1, springs, damaged, cache)
                    };
                    res_dot + res_hash
                },
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    };
    // cache.insert  ((previous,i,j), out);
    out
}



pub fn process_part1(input: &str) -> i32 {
    let out = input
        .split("\n")
        .map(|line| {
            let (springs_str, damaged_str) = line.split_once(' ').unwrap();
            let springs = springs_str.chars().collect::<Vec<char>>();
            let mut damaged = vec![0];
            damaged.extend(damaged_str.split(',').map(|nr| nr.parse().unwrap()).collect::<Vec<usize>>());
            println!("{}", springs.len());
            println!("{:?} {:?}", springs, damaged);
            let mut cache: HashMap<(char, usize, usize), i32> = HashMap::new();
            let out = calculate('.', 0, 0, 0, &springs, &damaged, &mut cache );
            for (k,v) in cache {
                println!("{:?} {}", k, v);
            }
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



}