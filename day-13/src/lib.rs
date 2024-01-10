pub fn check_reflection(v: Vec<String>) -> i64 {
    let mut reflex_candidates = vec![];
    for i in 0..v.len()-1 {
        if v[i] == v[i+1] {
            reflex_candidates.push(i);
        }
    }
    for reflex_start in reflex_candidates {
        let mut reflexed = true;
        let mut idx: usize = 1;
        while reflex_start as isize - idx as isize >= 0 && reflex_start + idx + 1 < v.len() {
            if v[reflex_start - idx] != v[reflex_start + idx + 1] {
                reflexed = false;
                break;
            } else {
                idx += 1;
            };
        };
        if reflexed == true {
            return reflex_start as i64;
        }
    }
    -1
}

pub fn evaluate_grid(grid: &str) -> (i64, char) {
    let v_horizontal: Vec<String> = grid
        .split('\n')
        .map(|element| element.to_string())
        .collect();
    let h_reflex = check_reflection(v_horizontal.clone());
    if h_reflex != -1 {
        return (100 * (h_reflex + 1),'h');
    } else {

        let v_vertical: Vec<String> = (0..v_horizontal[0].len())
            .map(|col| {  v_horizontal.iter().map(|line| &line[col..col + 1]).collect() })
            .collect();

        let v_reflex = check_reflection(v_vertical);
        (v_reflex + 1, 'v')
    }
}



pub fn process_part1(input: &str) -> i64 {
    input
        .split("\n\n")
        .map(|grid| {
            evaluate_grid(grid).0
        })
        .sum()
}

//////////////////////////////////

pub fn compare_lines(line1: &str, line2: &str) -> bool {
    let mut count_smudges = 0;
    let line1_chars = line1.as_bytes();
    let line2_chars = line2.as_bytes();
    for idx in 0..line1.len() {
        if  line1_chars[idx] != line2_chars[idx]{
            count_smudges += 1;
        }
    }
    if count_smudges <= 1 {
        return true;
    }
    false
}

pub fn check_reflection_smudge(v: Vec<String>, dir: char, part1: &(i64,char)  )  -> i64 {
    let mut reflex_candidates = vec![];
    for i in 0..v.len()-1 {
        if compare_lines(&v[i],&v[i+1]) {
            reflex_candidates.push(i);
        }
    }
    for reflex_start in reflex_candidates {
        let mut reflexed = true;
        let mut idx: usize = 1;
        while reflex_start as isize - idx as isize >= 0 && reflex_start + idx + 1 < v.len() {
            if compare_lines(&v[reflex_start - idx],&v[reflex_start + idx + 1]) == false {
                reflexed = false;
                break;
            } else {
                idx += 1;
            };
        };
        if reflexed == true {
            match (dir, part1.1) {
                ('h' , 'h') => {
                    if reflex_start != (part1.0 as usize / 100  ) - 1 {
                       return reflex_start as i64;
                    }
                },
                ('v' , 'v') => {
                    if reflex_start !=  part1.0 as usize - 1 {
                    return reflex_start as i64;
                }},
                (_,_) => {return reflex_start as i64;}
            }
        }
    }
    -1
}

pub fn process_part2(input: &str) -> i64 {
    input
        .split("\n\n")
        .map(|grid| {
            let v_horizontal: Vec<String> = grid
                .split('\n')
                .map(|element| element.to_string())
                .collect();
            let part1 = evaluate_grid(grid);
            let h_reflex = check_reflection_smudge(v_horizontal.clone(), 'h', &part1);
            if h_reflex != -1 {
                return 100 * (h_reflex + 1);
            } else {
                let v_vertical: Vec<String> = (0..v_horizontal[0].len())
                    .map(|col| {  v_horizontal.iter().map(|line| &line[col..col + 1]).collect() })
                    .collect();

                let v_reflex = check_reflection_smudge(v_vertical, 'v', &part1);
                v_reflex + 1
            }
        })
        .sum()
}



////////////////////////////////////////


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_first() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        let result = process_part1(&input);
        assert_eq!(result, 405);
    }
    #[test]
    fn test_second() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        let result = process_part2(&input);
        assert_eq!(result, 400);
    }

}