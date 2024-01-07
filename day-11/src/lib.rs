use std::collections::HashSet;

pub fn calc_distance(first: (usize, usize), second: (usize, usize) ) -> i64 {
    ((first.0 as i64 - second.0 as i64).abs() + (first.1 as i64 - second.1 as i64).abs()) as i64
}

pub fn process_part1(input: &str) -> i64 {
    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    let space = input
        .split("\n")
        .map(|line| {
            line.chars().collect::<Vec<char>>()
        }).collect::<Vec<Vec<char>>>();
    let mut rows: HashSet<usize> = (0..space.len()).collect();
    let mut columns: HashSet<usize> = (0..space[0].len()).collect();
    for i in 0..space.len() {
        for j in 0..space[0].len() {
            if space[i][j] == '#' {
                galaxies.push((i,j));
                rows.remove(&i);
                columns.remove(&j);
            }
        }
    }
    let mut rows = rows.into_iter().collect::<Vec<_>>();
    let mut columns = columns.into_iter().collect::<Vec<_>>();
    rows.sort();
    columns.sort();
    for galaxy in galaxies.iter_mut() {
        let mut idx = 0;
        let mut rows_iter = rows.iter();
        let mut rows_next = rows_iter.next();
        loop {
            if let Some(next) = rows_next {
                if galaxy.0 > *next {
                    idx += 1;
                    rows_next = rows_iter.next();
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        galaxy.0 += idx;

        idx = 0;
        let mut columns_iter = columns.iter();
        let mut columns_next = columns_iter.next();
        loop {
            if let Some(next) = columns_next {
                if galaxy.1 > *next {
                    idx += 1;
                    columns_next = columns_iter.next();
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        galaxy.1 += idx;
    }
    let mut sum: i64 = 0;
    for i in 0..galaxies.len() {
        for j in i+1..galaxies.len() {
            sum += calc_distance(galaxies[i],galaxies[j]);
        }
    }
    sum
}

///////////

pub fn process_part2(input: &str, offset: usize) -> i64 {
    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    let space = input
        .split("\n")
        .map(|line| {
            line.chars().collect::<Vec<char>>()
        }).collect::<Vec<Vec<char>>>();
    let mut rows: HashSet<usize> = (0..space.len()).collect();
    let mut columns: HashSet<usize> = (0..space[0].len()).collect();
    for i in 0..space.len() {
        for j in 0..space[0].len() {
            if space[i][j] == '#' {
                galaxies.push((i,j));
                rows.remove(&i);
                columns.remove(&j);
            }
        }
    }
    let mut rows = rows.into_iter().collect::<Vec<_>>();
    let mut columns = columns.into_iter().collect::<Vec<_>>();
    rows.sort();
    columns.sort();
    for galaxy in galaxies.iter_mut() {
        let mut idx = 0;
        let mut rows_iter = rows.iter();
        let mut rows_next = rows_iter.next();
        loop {
            if let Some(next) = rows_next {
                if galaxy.0 > *next {
                    idx += 1;
                    rows_next = rows_iter.next();
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        galaxy.0 += idx * (offset - 1);

        idx = 0;
        let mut columns_iter = columns.iter();
        let mut columns_next = columns_iter.next();
        loop {
            if let Some(next) = columns_next {
                if galaxy.1 > *next {
                    idx += 1;
                    columns_next = columns_iter.next();
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        galaxy.1 += idx * (offset - 1);
    }
    let mut sum: i64 = 0;
    for i in 0..galaxies.len() {
        for j in i+1..galaxies.len() {
            sum += calc_distance(galaxies[i],galaxies[j]);
        }
    }
    sum
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

    #[test]
    fn test_second_1() {
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
        let result = process_part2(&input, 10);
        assert_eq!(result, 1030);
    }

    #[test]
    fn test_second_2() {
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
        let result = process_part2(&input, 100);
        assert_eq!(result, 8410);
    }
}