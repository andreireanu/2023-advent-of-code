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

pub fn process_part1(input: &str) -> i64 {
    input
        .split("\n\n")
        .map(|grid| {
            let mut v_horizontal: Vec<String> = Vec::new();
            v_horizontal = grid
                .split('\n')
                .map(|element| element.to_string())
                .collect();
            let h_reflex = check_reflection(v_horizontal.clone());
            if h_reflex != -1 {
                return 100 * (h_reflex + 1);
            } else {

            let v_vertical: Vec<String> = (0..v_horizontal[0].len())
                .map(|col| {  v_horizontal.iter().map(|line| &line[col..col + 1]).collect() })
                .collect();

            let v_reflex = check_reflection(v_vertical);
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

}