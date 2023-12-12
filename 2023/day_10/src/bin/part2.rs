use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = parse(input);
    dbg!(output);
}

fn parse(input: &str) -> i32 {
    
    let mut start_pos = vec![0,0];
    let mut count = 0;
    
    let mut grid = vec![];
    for (row, line) in input.lines().enumerate() {
        let mut grid_row = vec![];
        for (col, ch) in line.chars().enumerate() {
            grid_row.push(ch);
            if ch == 'S' {
                start_pos[0] = row;
                start_pos[1] = col;
            }
        }
        grid.push(grid_row);
    }

    let mygrid = part1_parse(input);
     
    for i in 0..grid.len() {
        let mut inside = false;
        for j in 0..grid[i].len() {
            if is_pipe(grid[i][j]) && mygrid[i][j] {
                inside = !inside;
            }
            if inside && !mygrid[i][j] && grid[i][j] != 'S' {
                count += 1;
            } else {
            }
        }
    }



    return count;
}

fn part1_parse(input: &str) -> Vec<Vec<bool>> {

    let mut start_pos = vec![0,0];
    let mut distance = 0;
    
    let mut grid = vec![];
    for (row, line) in input.lines().enumerate() {
        let mut grid_row = vec![];
        for (col, ch) in line.chars().enumerate() {
            grid_row.push(ch);
            if ch == 'S' {
                start_pos[0] = row;
                start_pos[1] = col;
            }
        }
        grid.push(grid_row);
    }
    let mut out = vec![];
    for i in 0..grid.len() {
        let mut inner = vec![];
        for _ in 0..grid[i].len() {
            inner.push(false);
        }
        out.push(inner);
    }
   
    let mut current_pos = copy_vec(&start_pos);
    
    let mymap = generate_pipe_to_dir_map();

    // finds neighboring pipe that points to s, distance += 1, and current_pos = that pipe
    find_first_non_s_pipe(&mut current_pos, &mut distance, &grid, &mymap);


    out[current_pos[0]][current_pos[1]] = true;
    let mut previous_position = copy_vec(&start_pos);

    while !is_same_position(&start_pos, &current_pos) {

        
        out[current_pos[0]][current_pos[1]] = true;
        find_next_position(&mut current_pos, &mut distance, &mut previous_position, &grid, &mymap);     
    }

    return out;
}

fn is_pipe(ch: char) -> bool {
    let val = match ch {
        'L' => true,
        '|' => true,
        'J' => true,
        _ => false
    };
    return val;
}

fn find_next_position(
    current_pos: &mut Vec<usize>, 
    distance: &mut i32, 
    previous_position: &mut Vec<usize>,
    grid: &Vec<Vec<char>>, 
    map: &HashMap<char, Vec<(i32, i32)>>) 
{
    *distance += 1;
    for (i, j) in &map[&grid[current_pos[0]][current_pos[1]]] {
        if current_pos[0] as i32 + i == previous_position[0] as i32 {
            if current_pos[1] as i32 + j == previous_position[1] as i32 {
                continue;
            }
        }
        previous_position[0] = current_pos[0];
        previous_position[1] = current_pos[1];
        current_pos[0] = (current_pos[0] as i32 + i) as usize;
        current_pos[1] = (current_pos[1] as i32 + j) as usize;
        return;
    }
}
fn generate_pipe_to_dir_map() -> HashMap<char, Vec<(i32, i32)>> {
    let mut map = HashMap::new();
    map.insert('|', vec![(-1,0), (1,0)]);
    map.insert('L', vec![(-1,0), (0,1)]);
    map.insert('-', vec![(0,-1), (0,1)]);
    map.insert('J', vec![(-1,0), (0,-1)]);
    map.insert('F', vec![(0,1), (1,0)]);
    map.insert('7', vec![(0,-1), (1, 0)]);
    return map;
}

fn find_first_non_s_pipe(
    current_pos: &mut Vec<usize>, 
    distance: &mut i32, 
    grid: &Vec<Vec<char>>, 
    map: &HashMap<char, Vec<(i32, i32)>>) 
{
    *distance += 1;
    for (i,j) in vec![(1,0), (-1,0), (0,-1), (0, 1)] {
        let row_index = current_pos[0] as i32 + i;
        let col_index = current_pos[1] as i32 + j;
        if row_index < 0 || row_index as usize >= grid.len() {
            continue;
        }
        if col_index < 0 || col_index as usize >= grid[0].len() {
            continue;
        }
        let my_char = grid[row_index as usize][col_index as usize];

        for (offset_x, offset_j) in map[&my_char].iter() {

            if offset_x + i == 0 {
                if offset_j + j == 0 {
                    current_pos[0] = row_index as usize;
                    current_pos[1] = col_index as usize;
                    return;
                }
            }
        }
    }
}

fn copy_vec(a: &Vec<usize>) -> Vec<usize> {
    let mut new_vec = vec![];
    for i in a {
        new_vec.push(*i);
    }
    return new_vec;
}

fn is_same_position(a: &Vec<usize>, b: &Vec<usize>) -> bool {
    for (i, j) in a.iter().zip(b) {
        if *i != *j {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn parse_example() {
        let result = parse(r".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
");
        assert_eq!(result, 8);
    }

    #[test]
    fn parse_example_2() {
        let result = parse(r"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
");
        assert_eq!(result, 10);
    }
}
