use std::collections::HashMap;


// .??..??...?##. 1,1,3

pub fn calculate(i: usize, j: usize, current_damaged: usize, springs: &Vec<char>, damaged: &Vec<usize>, hm: &mut HashMap<(usize, usize), i32> ) -> i32 {
    if let Some(val) = hm.get(&(i,j)) {
        return *val;
    }
    if current_damaged > damaged[j] {
        return 0;
    }
    if i == springs.len() - 1 {
        return 1;
    }

    let out = match (springs.get(i-1).unwrap(), springs.get(i).unwrap()) {
            ('.', '.') => calculate(i+1, 0, 0, &springs, &damaged, hm ),
            ('.', '#') => calculate(i+1, j+1, 1 , &springs, &damaged, hm ),
            ('#', '.') => calculate(i+1, j, 0, &springs, &damaged, hm ),
            ('#', '#') => calculate(i+1, j, current_damaged+1, &springs, &damaged, hm ),
            ('?', '.') => calculate(i+1, 0, 0, &springs, &damaged, hm ) + calculate(i+1, j, 0, &springs, &damaged, hm ),
            ('?', '#') => calculate(i+1, j+1, 1 , &springs, &damaged, hm ) + calculate(i+1, j, current_damaged+1, &springs, &damaged, hm ),
            ('.' , '?') => calculate(i+1, 0, 0, &springs, &damaged, hm ) + calculate(i+1, j+1, 1 , &springs, &damaged, hm ),
            ('#' , '?') => calculate(i+1, j, 0, &springs, &damaged, hm ) + calculate(i+1, j, current_damaged+1, &springs, &damaged, hm ),
            ('?' , '?') => calculate(i+1, 0, 0, &springs, &damaged, hm ) + calculate(i+1, j, 0, &springs, &damaged, hm ) + calculate(i+1, j+1, 1 , &springs, &damaged, hm ) + calculate(i+1, j, current_damaged+1, &springs, &damaged, hm ) + calculate(i+1, 0, 0, &springs, &damaged, hm ) + calculate(i+1, j+1, 1 , &springs, &damaged, hm ) + calculate(i+1, j, 0, &springs, &damaged, hm ) + calculate(i+1, j, current_damaged+1, &springs, &damaged, hm ) ,
        (_,_) => unreachable!()
        };
    hm.insert((i,j), out);
    println!("{} {} {} -> {}", i, j, current_damaged, out);

    out
}




pub fn process_part1(input: &str) -> i32 {
    input
        .split("\n")
        .for_each(|line| {
            let (springs_str, damaged) = line.split_once(' ').unwrap();
            let mut springs = vec!['.'];
            springs.extend(springs_str.chars().collect::<Vec<char>>());
            let damaged = damaged.split(',').map(|nr| nr.parse().unwrap()).collect::<Vec<usize>>();
            println!("{}", springs.len());
            println!("{:?} {:?}", springs, damaged);
            let mut hm: HashMap<(usize, usize), i32> = HashMap::new();
            calculate(1, 0, 0, &springs, &damaged, &mut hm);
            println!("{:?}", hm);

        });
    0
}


////////////////////////////////////////


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_first() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let result = process_part1(&input);
        assert_eq!(result, 374);
    }



}