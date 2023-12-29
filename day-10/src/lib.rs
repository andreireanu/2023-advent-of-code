pub enum Dir {
    Up, Down, Left, Right
}
use Dir::*;

pub fn check_neighb( grid: &Vec<Vec<char>>, visited: &mut Vec<Vec<i32>>,row: usize, col: usize, dir: Dir, depth: i32) -> Option<(usize, usize)> {
    match dir {
        Up => {
            match row.checked_sub(1) {
                Some(_) => {},
                None => return None
            }
            if visited[row-1][col] != 0 { return None };
            match grid[row - 1][col] {
                '|' | '7' | 'F' => {
                    visited[row-1][col] = depth;
                    return Some((row-1,col)); },
                _ => return None,
            }
        },
        Down => {
            if row + 1 >= grid.len() {return None};
            if visited[row+1][col] != 0 { return None };
            match grid[row+1][col] {
                '|' | 'L' | 'J' => {
                    visited[row+1][col] = depth;
                    return Some((row+1,col)); },
                _ => return None,
            }
        },
        Left => {
            match col.checked_sub(1) {
                Some(_) => {},
                None => return None
            }
            if visited[row][col-1] != 0 { return None };
            match grid[row][col-1] {
                '-' | 'L' | 'F' => {
                    visited[row][col-1] = depth;
                    return Some((row,col-1)); },
                _ => return None,
            }
        },
        Right => {
            if col + 1 >= grid[0].len() {return None};
            if visited[row][col+1] != 0 { return None };
            match grid[row][col+1] {
                '-' | 'J' | '7' => {
                    visited[row][col+1] = depth;
                    return Some((row,col+1)); },
                _ => return None,
            }
        },
    }
}

pub fn forward_grid(neighb: Vec<(usize, usize)>, grid: &Vec<Vec<char>>, visited: &mut Vec<Vec<i32>>, depth: i32) -> Vec<(usize, usize)> {
    let mut new_neighb: Vec<(usize, usize)> = vec![];
    for current in neighb {
        let row = current.0;
        let col = current.1;
        if let Some(loop_neighb) = check_neighb(grid, visited, row, col, Up, depth) {
            new_neighb.push(loop_neighb);
        }
        if let Some(loop_neighb) = check_neighb(grid, visited, row, col, Down, depth) {
            new_neighb.push(loop_neighb);
        }
        if let Some(loop_neighb) = check_neighb(grid, visited, row, col, Left, depth) {
            new_neighb.push(loop_neighb);
        }
        if let Some(loop_neighb) = check_neighb(grid, visited, row, col, Right, depth) {
            new_neighb.push(loop_neighb);
        }
    }
    new_neighb
}

pub fn process_part1(input: &str) -> i32 {
     let grid: Vec<Vec<char>>  = input
         .split("\n")
         .map(|line| line.chars().collect())
         .collect();
     let mut visited =   vec![vec![0; grid[0].len()]; grid.len()];
     let mut neighb: Vec<(usize, usize)> = vec![];
     for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'S' {
                neighb.push((i,j));
                break;
            }
        }
     }
    let mut depth = 0;
    while neighb.len() > 0 {
        depth += 1;
        neighb = forward_grid(neighb, &grid,  &mut visited, depth);
    }
    depth - 1
}

////////////////////////////////////////


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_first() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let result = process_part1(&input);
        assert_eq!(result, 114);
    }



}