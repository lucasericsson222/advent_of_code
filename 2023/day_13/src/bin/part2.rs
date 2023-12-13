fn main() {
    let input = include_str!("./input.txt");
    let output = parse(input);
    dbg!(output);
}

fn parse(input: &str) -> usize {
    let mut sum = 0;

    for (i, input_section) in input.split("\n\n").enumerate() {
        let mut grid = vec![];
        for line in input_section.lines() {
            let mut row = vec![];
            for char in line.chars() {
                row.push(char);
            }
            grid.push(row);
        }
       
        let mut temp_sum = 0;
        for row in 0..grid.len() - 1 {
            if check_horizontal_mirror(&grid, row) {
                temp_sum += (row + 1) * 100;
            }
        }

        for col in 0..grid[0].len() - 1 {
            if check_vertical_mirror(&grid, col) {
                temp_sum += col + 1;
            }
        }


        sum += temp_sum;

    }

    return sum;
}

fn print_row(
    grid: &Vec<Vec<char>>,
    row: usize
) {
    for i in 0..grid[row].len() {
        print!("{}", grid[row][i]);
    }
    print!("\n");
}

fn check_vertical_mirror(
    grid: &Vec<Vec<char>>, 
    col: usize
) -> bool {
    let mut mistakes = 0; 
    // col = left side of divide
    // loop through left side of mirror
    for i in 0..=col {

        // calculate right side mirroring
        let dist = col - i;
        let mirror_col = col + 1 + dist;

        if mirror_col >= grid[0].len() {
            continue;
        }
        for row in 0..grid.len() {
            if grid[row][i] != grid[row][mirror_col] {
                mistakes += 1;
            }
        }
        
    }
    return mistakes == 1;
}

fn check_horizontal_mirror(
    grid: &Vec<Vec<char>>, 
    row: usize
) -> bool {

    let mut mistakes = 0;
    for i in 0..=row {

        let dist = row - i;
        let mirror_row = row + 1 + dist;

        if mirror_row >= grid.len() {
            continue;
        }


        for col in 0..grid[0].len() {
            if grid[i][col] != grid[mirror_row][col] {
                mistakes += 1;
            }
        }
    }
    
    return mistakes == 1;
}

#[cfg(test)]
mod tests {
}
