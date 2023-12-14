fn main() {
    let input = include_str!("./input.txt");
    let output = parse(input);
    dbg!(output);
}

fn parse(input: &str) -> usize {
    let mut sum = 0;

    let mut grid = str_to_grid(input);

    let mut previous_cycles = vec![]; 
   
    let index;
    loop {
        tilt_north(&mut grid);
        tilt_west(&mut grid);
        tilt_south(&mut grid);
        tilt_east(&mut grid);
        if let Some(previous_index) = tilt_in_previous_cycles(&grid, &previous_cycles) {
            println!("Current_index: {}", previous_cycles.len());
            println!("Cycle_index: {}", previous_index);
            index = previous_index;
            previous_cycles.push(grid.clone());
            break;
        }
        previous_cycles.push(grid.clone());
    }
    
    print_grid(&previous_cycles[previous_cycles.len() - 1]);
    println!("-------------");
    print_grid(&previous_cycles[index]);

    let cycle_length = previous_cycles.len() - 1 - index;
    println!("Cycle_Length: {}", cycle_length);
    let new_billion_position = (1000000000 - index - 1) % cycle_length + index;

    println!("new_billion_position {}", new_billion_position);
    for (row_num, row) in previous_cycles[new_billion_position].iter().rev().enumerate() {
        for object in row {
            if *object == 'O' {
                sum += row_num + 1;
            }
        }
    }


    return sum;
}

fn tilt_in_previous_cycles(current: &Vec<Vec<char>>, olds: &Vec<Vec<Vec<char>>>) -> Option<usize> {
    for (id, old) in olds.iter().enumerate() {
        if old == current {
            return Some(id);
        }
         
    }
    return None;
}

fn tilt_north(grid: &mut Vec<Vec<char>>) {
    let mut moved_something = true; 
    while moved_something {
        moved_something = false;

        for row in 0..grid.len() {
            for col in 0..grid[row].len() {
                if grid[row][col] == 'O' {
                    if row as i32 - 1 >= 0 {
                        if grid[row-1][col] != '#' && grid[row-1][col] != 'O' {
                            grid[row][col] = '.';
                            grid[row-1][col] = 'O';
                            moved_something = true;
                        }
                    }
                }
            }
        }
    }
}

fn tilt_south(grid: &mut Vec<Vec<char>>) {
    let mut moved_something = true;
    while moved_something {
        moved_something = false;
        for row in 0..grid.len() {
            for col in 0..grid[row].len() {
                if grid[row][col] == 'O' {
                    if row + 1 < grid.len() {
                        if grid[row + 1][col] != '#' && grid[row + 1][col] != 'O' {
                            grid[row][col] = '.';
                            grid[row + 1][col] = 'O';
                            moved_something = true;
                        }
                    }
                }
            }
        }
    }
}

fn tilt_east(grid: &mut Vec<Vec<char>>) {
    let mut moved_something = true;
    while moved_something {
        moved_something = false;

        for row in 0..grid.len() {
            for col in 0..grid[row].len() {
                if grid[row][col] == 'O' {
                    if col + 1 < grid[row].len() {
                        if grid[row][col + 1] != '#' && grid[row][col + 1] != 'O' {
                            grid[row][col] = '.';
                            grid[row][col + 1] = 'O';
                            moved_something = true;
                        }
                    }
                }
            }
        }
    }
}

fn tilt_west(grid: &mut Vec<Vec<char>>) {
    let mut moved_something = true;
    while moved_something {
        moved_something = false;
        for row in 0..grid.len() {
            for col in 0..grid[row].len() {
                if grid[row][col] == 'O' {
                    if col as i32 - 1 >= 0 {
                        if grid[row][col - 1] != '#' && grid[row][col - 1] != 'O' {
                            grid[row][col] = '.';
                            grid[row][col - 1] = 'O';
                            moved_something = true;
                        }
                    }
                }
            }
        }
    }
}

fn str_to_grid(input: &str) -> Vec<Vec<char>> {
    let mut grid = vec![];

    for line in input.lines() {
        let mut inter = vec![]; 
        for ch in line.chars() {
            inter.push(ch);
        }
        grid.push(inter);
    }

    return grid;
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        for item in row {
            print!("{}", item);
        }
        print!("\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn parse_example() {
        let result = parse(r"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
");
        assert_eq!(result, 64);
    }
}
