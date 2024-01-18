pub fn move_rock_north(i: usize, j: usize, grid: &mut Vec<Vec<char>>) {
    let mut k = i;
    while k >= 1 {
        match grid[k-1][j]  {
            '.' => { k -= 1; } ,
            'O' | '#' => break,
            _ => unreachable!()
        }
    }
    if k != i {
        grid[k][j] = 'O';
        grid[i][j] = '.';
    }
}

pub fn move_rock_west(i: usize, j: usize, grid: &mut Vec<Vec<char>>) {
    let mut k = j;
    while k >= 1 {
        match grid[i][k-1]  {
            '.' => { k -= 1;
            } ,
            'O' | '#' => break,
            _ => unreachable!()
        }
    }
    if k != j {
        grid[i][k] = 'O';
        grid[i][j] = '.';
    }
}

pub fn move_rock_south(i: usize, j: usize, grid: &mut Vec<Vec<char>>) {
    let mut k = i;
    while k < grid.len() - 1 {
        match grid[k+1][j]  {
            '.' => { k += 1; } ,
            'O' | '#' => break,
            _ => unreachable!()
        }
    }
    if k != i {
        grid[k][j] = 'O';
        grid[i][j] = '.';
    }
}

pub fn move_rock_east(i: usize, j: usize, grid: &mut Vec<Vec<char>>) {
    let mut k = j;
    while k < grid.len() - 1 {
        match grid[i][k+1]  {
            '.' => { k += 1;
            } ,
            'O' | '#' => break,
            _ => unreachable!()
        }
    }
    if k != j {
        grid[i][k] = 'O';
        grid[i][j] = '.';
    }
}

pub fn cycle(grid: &mut Vec<Vec<char>>) {

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'O' {
                move_rock_north(i, j, grid);
            }
        }
    }
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'O' {
                move_rock_west(i, j, grid);
            }
        }
    }
    for i in (0..grid.len()).rev() {
        for j in (0..grid[0].len()).rev()  {
            if grid[i][j] == 'O' {
                move_rock_south(i, j, grid);
            }
        }
    }
    for i in (0..grid.len()).rev() {
        for j in (0..grid[0].len()).rev()  {
            if grid[i][j] == 'O' {
                move_rock_east(i, j, grid);
            }
        }
    }


}


pub fn process_part1(input: &str) -> i64 {
    let mut grid = input
      .split('\n')
      .map(|line|
           line.chars().collect::<Vec<char>>())
      .collect::<Vec<Vec<char>>>();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'O' {
                move_rock_north(i, j, &mut grid);
            }
        }
    }
    let sum: usize = grid.iter().enumerate().map(|(i, row)| {
        let row_sum = row.iter().filter(|&&cell| cell == 'O').count();
        row_sum * (grid.len() - i)
    }).sum();
    sum as i64
}

pub fn process_part2(input: &str) -> i64 {
    let mut grid = input
        .split('\n')
        .map(|line|
            line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut sum = 0;
    // period is 59
    for i in 0..268 {
        cycle(&mut grid);
        sum = grid.iter().enumerate().map(|(i, row)| {
            let row_sum = row.iter().filter(|&&cell| cell == 'O').count();
            row_sum * (grid.len() - i)
        }).sum();
        println!("{}", sum);
    }
    sum as i64
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