use std::{collections::{HashMap, HashSet}, io};

fn main() {
    let input = include_str!("./input1.txt");
    let output = parse(input);
    dbg!(output);
}

fn parse(input: &str) -> usize {
    let (grid_input, dir_input) = input.split_once("\n\n").unwrap();

    let mut grid = vec![];

    for line in grid_input.lines() {
        let mut row = vec![];
        for char in line.chars() {
            if char == '#' {
                row.push('#');
                row.push('#');
            } else if char == 'O' {
                row.push('[');
                row.push(']');
            } else if char == '@' {
                row.push('@');
                row.push('.');
            } else {
                row.push('.');
                row.push('.');
            }
        }
        grid.push(row);
    }

    let dir_input = dir_input.replace("\n", "");
    let mut pos = find_char_pos(&grid);
    let mut filler = String::new();
    for dir_char in dir_input.chars() {
        let dir = map_char_to_offset(dir_char);

        //print_grid(&grid);
        let res = try_move(&grid, dir, pos, '@');
        if let Some(mut poss) = res {
            let new_pos = ((pos.0 as i32 + dir.0) as usize, (pos.1 as i32 + dir.1) as usize);
            poss.insert(pos);
            apply_move(&mut grid, poss, dir);
            pos = new_pos;
        }
        //io::stdin()
        //    .read_line(&mut filler)
        //    .expect("Failed to read line");
    }
    //print_grid(&grid);
    let mut sum = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '[' {
                sum += 100 * i + j;
            }
        }
    }
    return sum;
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            print!("{}", grid[i][j]);
        }
        println!();
    }
}

fn find_char_pos(grid: &Vec<Vec<char>>) -> (usize, usize) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '@' {
                return (i,j);
            }
        }
    }
    panic!("Could not find @");
}

fn apply_move(grid: &mut Vec<Vec<char>>, positions: HashSet<(usize, usize)>, dir: (i32, i32)) {
    let mut vals = HashMap::new();

    for pos in &positions {
        //print!("{}", grid[pos.0][pos.1]);
        vals.insert(pos, grid[pos.0][pos.1].clone());
        grid[pos.0][pos.1] = '.';
    }
    //println!();
    for pos in &positions {
        let new_pox_x = (pos.0 as i32 + dir.0) as usize;
        let new_pox_y = (pos.1 as i32 + dir.1) as usize;
        grid[new_pox_x][new_pox_y] = vals[&pos];
    }
}

fn try_move(grid: &Vec<Vec<char>>, dir: (i32, i32), pos: (usize, usize), ch: char) -> Option<HashSet<(usize, usize)>> {
    let new_pos_x = pos.0 as i32 + dir.0;
    let new_pos_y = pos.1 as i32 + dir.1;

    let new_pos_tuple = (new_pos_x as usize, new_pos_y as usize);
    
    let mut pos_to_move = HashSet::new();

    if new_pos_x < 0 || new_pos_x >= grid.len() as i32 {
        return None;
    }
    if new_pos_y < 0 || new_pos_y >= grid[0].len() as i32 {
        return None;
    }
    if grid[new_pos_x as usize][new_pos_y as usize] == '#' {
        return None; 
    }
    let new_pos_char = grid[new_pos_x as usize][new_pos_y as usize];
    if new_pos_char == '[' || new_pos_char == ']' {
        let tmp_pos = (new_pos_x as usize, new_pos_y as usize);

        if dir.1 != 0 {
            pos_to_move.extend(try_move(grid, dir, tmp_pos, grid[tmp_pos.0][tmp_pos.1])?.iter());
            pos_to_move.insert(new_pos_tuple);
        } else {
            if grid[tmp_pos.0][tmp_pos.1] == ']' {
                pos_to_move.extend(try_move(grid, dir, (tmp_pos.0, tmp_pos.1 - 1), grid[tmp_pos.0][tmp_pos.1 - 1])?.iter());
                pos_to_move.extend(try_move(grid, dir, (tmp_pos.0, tmp_pos.1), grid[tmp_pos.0][tmp_pos.1])?.iter());
                pos_to_move.insert(new_pos_tuple);
                pos_to_move.insert((tmp_pos.0, tmp_pos.1 - 1));
            } else {
                pos_to_move.extend(try_move(grid, dir, (tmp_pos.0, tmp_pos.1 + 1), grid[tmp_pos.0][tmp_pos.1 + 1])?.iter());
                pos_to_move.extend(try_move(grid, dir, (tmp_pos.0, tmp_pos.1), grid[tmp_pos.0][tmp_pos.1])?.iter());
                pos_to_move.insert(new_pos_tuple);
                pos_to_move.insert((tmp_pos.0, tmp_pos.1 + 1));
            }
            
        }
    }

    return Some(pos_to_move);
}

fn map_char_to_offset(dir_char: char) -> (i32, i32) {
    match dir_char {
        '<' => (0, -1),
        '>' => (0, 1),
        '^' => (-1, 0),
        'v' => (1, 0),
        _ => panic!("What you doing?")
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example() {
        let result = parse(r"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
");
        assert_eq!(result, 1930);
    }

}
