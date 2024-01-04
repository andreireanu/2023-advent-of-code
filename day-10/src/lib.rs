#[derive(Debug)]
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
                visited[i][j] = 1;
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

///////////

pub fn check_neighb2( grid: &mut Vec<Vec<char>>, visited: &mut Vec<Vec<i32>>,row: usize, col: usize, dir: Dir, depth: i32) -> Option<(usize, usize)> {
    match dir {
        Up => {
            match row.checked_sub(1) {
                Some(_) => {},
                None => return None
            }
            if visited[row-1][col] != 0 { return None };
            match grid[row - 1][col] {
                '|'  => grid[row-1][col] = '║',
                 '7' => grid[row-1][col] = '╗',
                 'F' => grid[row-1][col] = '╔',
                _ => return None,
            }
            visited[row-1][col] = depth;
            return Some((row-1,col));
        },
        Down => {
            if row + 1 >= grid.len() {return None};
            if visited[row+1][col] != 0 { return None };
            match grid[row+1][col] {
                '|' =>  grid[row+1][col] = '║',
                 'L' => grid[row+1][col] = '╚',
                 'J' => grid[row+1][col] = '╝',
                _ => return None,
            }
            visited[row+1][col] = depth;
            return Some((row+1,col));
        },
        Left => {
            match col.checked_sub(1) {
                Some(_) => {},
                None => return None
            }
            if visited[row][col-1] != 0 { return None };
            match grid[row][col-1] {
                '-' => grid[row][col-1] = '═' ,
                 'L' => grid[row][col-1] = '╚',
                 'F' => grid[row][col-1] = '╔',
                _ => return None,
            }
            visited[row][col-1] = depth;
            return Some((row,col-1));
        },
        Right => {
            if col + 1 >= grid[0].len() {return None};
            if visited[row][col+1] != 0 { return None };
            match grid[row][col+1] {
                '-' => grid[row][col+1] = '═',
                 'J' => grid[row][col+1] = '╝',
                 '7' => grid[row][col+1] = '╗',
                _ => return None,
            }
            visited[row][col+1] = depth;
            return Some((row,col+1));
        },
    }
}



pub fn forward_grid2(neighb: Vec<(usize, usize)>, grid: &mut Vec<Vec<char>>, visited: &mut Vec<Vec<i32>>, depth: i32) -> Vec<(usize, usize)> {
    let mut new_neighb: Vec<(usize, usize)> = vec![];
    for current in neighb {
        let row = current.0;
        let col = current.1;
        if let Some(loop_neighb) = check_neighb2(grid, visited, row, col, Up, depth) {
            new_neighb.push(loop_neighb);
        }
        if let Some(loop_neighb) = check_neighb2(grid, visited, row, col, Down, depth) {
            new_neighb.push(loop_neighb);
        }
        if let Some(loop_neighb) = check_neighb2(grid, visited, row, col, Left, depth) {
            new_neighb.push(loop_neighb);
        }
        if let Some(loop_neighb) = check_neighb2(grid, visited, row, col, Right, depth) {
            new_neighb.push(loop_neighb);
        }
    }
    new_neighb
}


pub fn match_neighb(i: usize, j: usize, dir: Dir, grid: & Vec<Vec<char>>) -> bool {
    match dir {
        Up => {
            match (grid[i][j],grid[i-1][j]) {
                ('║' | '╝' | '╚',  '║' | '╔' |'╗') => return true,
                (_, 'S') => return true,
                (_,_) => return false,
            }
        }
        Down => {
            match (grid[i][j],grid[i+1][j]) {
                ('║' | '╔' | '╗',  '║' | '╝' |'╚') => return true,
                (_, 'S') => return true,
                (_,_) => return false,
            }
        }
        Left => {
            match (grid[i][j],grid[i][j-1]) {
                ('═' | '╝' | '╗',  '═' | '╔' |'╚') => return true,
                (_, 'S') => return true,
                (_,_) => return false,
            }
        }
        Right => {
            match (grid[i][j],grid[i][j+1]) {
                ('═' | '╔' | '╚',  '═' | '╝' |'╗') => return true,
                (_, 'S') => return true,
                (_,_) => return false,
            }
        }
    }
    unreachable!()
}

pub fn validate_neighb(i: usize, j: usize, visited: &mut Vec<Vec<i32>>, grid: &mut Vec<Vec<char>>) -> bool {
    let val = visited[i][j];
    if i > 0 {
        let up = visited[i-1][j];
        if val - up == 1 {
            if match_neighb(i, j,Up, & grid) == true {
                return true;
            };
        }
    }
    if i < visited.len() - 1 {
        let down = visited[i+1][j];
        if val - down == 1 {
            if match_neighb(i, j,Down, & grid) == true {
                return true;
            };
        }
    }
    if j > 0 {
        let left = visited[i][j-1];
        if val - left == 1 {
            if match_neighb(i, j,Left, & grid) == true {
                return true;
            };
        }
    }
    if j < visited[0].len() - 1 {
        let right = visited[i][j+1];
        if val - right == 1 {
            if match_neighb(i, j,Right, &grid) == true {
                return true;
            };
        }
    }
    false
}


pub fn clear_extra_pipes(grid: &mut Vec<Vec<char>>, visited: &mut Vec<Vec<i32>>) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if visited[i][j] > 1 {
                 if validate_neighb(i, j, visited , grid) == false {
                     visited[i][j] = 0;
                     grid[i][j] = '.';
                 }
            }
            if visited[i][j] == 0 {
                grid[i][j] = '.';
            }
        }
    }
}

// pub fn evaluate_tile(grid: &mut Vec<Vec<char>>, visited: &mut Vec<Vec<i32>>) -> i32 {
//
// }


pub fn process_part2(input: &str, s: char) -> i32 {
    let mut grid: Vec<Vec<char>> = input
        .split("\n")
        .map(|line| line.chars().collect())
        .collect();
    let mut visited = vec![vec![0; grid[0].len()]; grid.len()];
    let mut neighb: Vec<(usize, usize)> = vec![];
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'S' {
                neighb.push((i, j));
                visited[i][j] = 1;
                grid[i][j] = s;
                break;
            }
        }
    }
    let mut depth = 1;
    while neighb.len() > 0 {
        depth += 1;
        neighb = forward_grid2(neighb, &mut grid, &mut visited, depth);
    }

    clear_extra_pipes(&mut grid, &mut visited);

    for i in 0..visited.len() {
        for j in 0..visited[1].len() {
            print!("{:2}  ", visited[i][j]);
        }
        println!("");
    }
    println!("");

    for i in 0..grid.len() {
        for j in 0..grid[1].len() {
            print!("{:2}  ", grid[i][j]);
        }
        println!("");
    }
    println!("");

    depth
}



////////////////////////////////////////


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_first() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        let result = process_part1(&input);
        assert_eq!(result, 8);
    }



}