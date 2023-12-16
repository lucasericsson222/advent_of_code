use std::{mem::swap, vec};

fn main() {
    let input = include_str!("./input.txt");
    let output = parse(input);
    dbg!(output);
}

fn parse(input: &str) -> usize {
    
    let mut max = 0;

    let mut grid = vec![];
    for line in input.lines() {
        let mut row = vec![];
        for ch in line.chars() {
            row.push(ch);
        }
        grid.push(row);
    }
    
    let mut edges = vec![];
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if x == 0 || y == 0 || x == grid[0].len() - 1 || y == grid.len() - 1 {
                edges.push((x as i32, y as i32));
            }
        }
    }

    for edge in edges {
        for dir in posible_dirs(edge, &grid) {
            
            let mut out_grid = vec![];

            for line in input.lines() {
                let mut out_row = vec![];
                for _ in line.chars() {
                    out_row.push(('.', vec![]));
                }
                out_grid.push(out_row);
            }

            ray(&grid, &mut out_grid, edge, dir);

            let mut sum = 0;

            for row in out_grid {
                for (ch, _) in row {
                    if ch == '#' {
                        sum += 1;
                    }
                }
            }

            if sum > max {
                max = sum;
            }

        }

    }

    return max;
}

fn posible_dirs(pos: (i32, i32), grid: &Vec<Vec<char>>) -> Vec<(i32, i32)> {
    let mut dirs = vec![];

    let (x, y) = pos;
    if x == 0 {
        dirs.push((1,0));
    }
    if x as usize == grid[0].len() - 1 {
        dirs.push((-1,0));
    }
    if y == 0 {
        dirs.push((0,1));
    }
    if y as usize == grid.len() - 1 {
        dirs.push((0, -1));
    }

    return dirs;
}

fn ray(
    grid: &Vec<Vec<char>>, 
    out_grid: &mut Vec<Vec<(char, Vec<(i32,i32)>)>>,
    pos: (i32, i32),
    dir: (i32, i32)
) {
    let (x, y) = pos;
    if y < 0 || y as usize >= grid.len() {
        return;
    }
    if x < 0 || x as usize >= grid[0].len() {
        return;
    }
    let (_, dirs) = &out_grid[y as usize][x as usize];
    if dirs.contains(&dir) {
        return;
    }
    out_grid[y as usize][x as usize].0 = '#'; 
    out_grid[y as usize][x as usize].1.push(dir);

    let current_char = grid[y as usize][x as usize];

    let (mut dir_x, mut dir_y) = dir;

    if current_char == '\\' {
        swap(&mut dir_x, &mut dir_y);
    }

    if current_char == '/' {
        dir_x *= -1;
        dir_y *= -1;
        swap(&mut dir_x, &mut dir_y);
    }

    if current_char == '-' {
        if dir_x == 0 {
            ray(grid, out_grid, (x + 1, y), (1, 0));
            ray(grid, out_grid, (x - 1, y), (-1, 0));
            return;
        }
    }

    if current_char == '|' {
        if dir_y == 0 {
            ray(grid, out_grid, (x, y + 1), (0, 1));
            ray(grid, out_grid, (x, y - 1), (0, -1));
            return;
        }
    }

    ray(grid, out_grid, (x + dir_x, y + dir_y), (dir_x, dir_y)); 
}

fn print_grid(grid: &Vec<Vec<(char, Vec<(i32, i32)>)>>) {
    for row in grid {
        for (ch, _) in row {
            print!("{}", ch);
        }
        println!();
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn parse_example() {
        let result = parse(r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
");
        assert_eq!(result, 51);
    }
}
