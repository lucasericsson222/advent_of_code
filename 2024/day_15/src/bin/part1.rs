use std::io;

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
            row.push(char);
        }
        grid.push(row);
    }

    let dir_input = dir_input.replace("\n", "");
    let mut pos = find_char_pos(&grid);
    let mut filler = String::new();
    for dir_char in dir_input.chars() {
        let dir = map_char_to_offset(dir_char);

        //print_grid(&grid);
        try_move(&mut grid, dir, &mut pos, '@');
        //io::stdin()
        //    .read_line(&mut filler)
        //    .expect("Failed to read line");
    }
    //print_grid(&grid);
    let mut sum = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'O' {
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

fn try_move(grid: &mut Vec<Vec<char>>, dir: (i32, i32), pos: &mut (usize, usize), ch: char) {
    let new_pos_x = pos.0 as i32 + dir.0;
    let new_pos_y = pos.1 as i32 + dir.1;

    let new_pos_tuple = (new_pos_x as usize, new_pos_y as usize);

    if new_pos_x < 0 || new_pos_x >= grid.len() as i32 {
        return;
    }
    if new_pos_y < 0 || new_pos_y >= grid[0].len() as i32 {
        return;
    }
    if grid[new_pos_x as usize][new_pos_y as usize] == '#' {
        return; 
    }
    if grid[new_pos_x as usize][new_pos_y as usize] == 'O' {
        let mut tmp_pos = (new_pos_x as usize, new_pos_y as usize);
        try_move(grid, dir, &mut tmp_pos, 'O');
        if tmp_pos == new_pos_tuple {
            return; 
        }
    }

    grid[pos.0][pos.1] = '.';
    *pos = (new_pos_x as usize, new_pos_y as usize);
    grid[pos.0][pos.1] = ch;
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
