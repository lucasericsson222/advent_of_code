use std::mem::swap;

fn main() {
    let input = include_str!("./input.txt");
    let output = parse(input);
    dbg!(output);
}

fn parse(input: &str) -> usize {
    
    let mut grid = vec![];
    
    let mut out_grid = vec![];

    for line in input.lines() {
        let mut row = vec![];
        let mut out_row = vec![];
        for ch in line.chars() {
            out_row.push(('.', vec![]));
            row.push(ch);
        }
        out_grid.push(out_row);
        grid.push(row);
    }

    ray(&grid, &mut out_grid, (0,0), (1,0));

    let mut sum = 0;

    for row in out_grid {
        for (ch, _) in row {
            if ch == '#' {
                sum += 1;
            }
        }
    }

    return sum;
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
        assert_eq!(result, 46);
    }
}
