use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = parse(input);
    dbg!(output);
}

fn parse(input: &str) -> i32 {
    
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
   
    let mut current_pos = copy_vec(&start_pos);
    
    let mymap = generate_pipe_to_dir_map();

    // finds neighboring pipe that points to s, distance += 1, and current_pos = that pipe
    find_first_non_s_pipe(&mut current_pos, &mut distance, &grid, &mymap);

    let mut previous_position = copy_vec(&start_pos);

    while !is_same_position(&start_pos, &current_pos) {

        println!("({},{}), {}", current_pos[0], current_pos[1], grid[current_pos[0]][current_pos[1]]);
        
        find_next_position(&mut current_pos, &mut distance, &mut previous_position, &grid, &mymap);     
    }

    return distance;
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
}
