use std::cmp;

fn main() {
    let input = include_str!("./input.txt");
    let output = parse(input);
    dbg!(output);
}

fn parse(input: &str) -> usize {
    let galaxy_expansion_rate = 1000000;
    let mut sum = 0;
    let mut empty_rows = vec![];
    let mut empty_cols = vec![];
    
    let mut grid = vec![];
    let mut galaxies = vec![];
    for (i, line) in input.lines().enumerate() {
        let mut inner = vec![];
        for (j, ch) in line.chars().enumerate()  {
            inner.push(ch); 
            if ch == '#' {
                galaxies.push((i,j));
            }
        }
        grid.push(inner);
    }

    for i in 0..grid.len() {
        let mut is_empty = true;
        for j in 0..grid[i].len() {
            if grid[i][j] == '#' {
                is_empty = false;
            }
        }
        if is_empty {
            empty_rows.push(i);
        }
    }

    for j in 0..grid[0].len() {
        let mut is_empty = true;
        for i in 0..grid.len() {
            if grid[i][j] == '#' {
                is_empty = false;
            }
        }
        if is_empty {
            empty_cols.push(j);
        }
    }

    for first in 0..galaxies.len() {
        for second in (first+1)..galaxies.len() {
            let (first_g_x, first_g_y) = galaxies[first];
            let (second_g_x, second_g_y) = galaxies[second];
            let mut basic_dist = second_g_y.abs_diff(first_g_y) + second_g_x.abs_diff(first_g_x);
            
            for row in &empty_rows {
                if cmp::min(&first_g_x, &second_g_x) < row && row < cmp::max(&first_g_x, &second_g_x) {
                    basic_dist += galaxy_expansion_rate - 1;
                }
            }
            for col in &empty_cols {
                if cmp::min(&first_g_y, &second_g_y) < col && col < cmp::max(&first_g_y, &second_g_y) {
                    basic_dist += galaxy_expansion_rate - 1;
                }
            }
            sum += basic_dist;
        }
    }

    return sum;

}

#[cfg(test)]
mod tests {
}
