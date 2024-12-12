use std::collections::HashSet;

fn main() {
    let input = include_str!("./input1.txt");
    let output = parse(input);
    dbg!(output);
}

fn parse(input: &str) -> usize {
    let mut grid = vec![];
    for line in input.lines() {
        let mut row = vec![];
        for ch in line.chars() {
            row.push(ch);
        }
        grid.push(row);
    }
    let mut sum: usize = 0;
    while exists_non_dot(&grid) {
        sum += calc_area_perim(&mut grid);


    }
    return sum;
}

fn exists_non_dot(grid: &Vec<Vec<char>>) -> bool {
    if let Some(a) = find_not_char_in_grid(grid, '.') {
        return true;
    }
    return false;
}

fn find_not_char_in_grid(grid: &Vec<Vec<char>>, ch: char) -> Option<(usize, usize)> {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] != ch {
                return Some((i, j));
            }
        }
    }
    return None;
}

fn calc_area_perim(grid: &mut Vec<Vec<char>>) -> usize {
    let start = find_not_char_in_grid(grid, '.').unwrap();    

    let mut cur_area = HashSet::new();
    flood_fill(grid, start, &mut cur_area);

    return cur_area.len() * perimeter(&cur_area);
}

fn perimeter(area: &HashSet<(usize, usize)>) -> usize {

    let mut sum = 0;

    for pos in area {
        for (ox, oy) in vec![(-1, 0), (1,0), (0, -1), (0, 1)] {
            let new_pos_x = pos.0 as i32 + ox;
            let new_pos_y = pos.1 as i32 + oy;
            
            if new_pos_x < 0 {
                continue;
            }
            if new_pos_y < 0 {
                continue;
            }

            if area.contains(&(new_pos_x as usize, new_pos_y as usize)) {
                sum += 1; 
            } 
        }
    }

    return 4 * area.len() - sum;
}

fn flood_fill(grid: &mut Vec<Vec<char>>, pos: (usize, usize), area: &mut HashSet<(usize, usize)>) {
    area.insert(pos);


    let prev = grid[pos.0][pos.1]; 
    grid[pos.0][pos.1] = '.';

    for (ox, oy) in vec![(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let new_pos_x = pos.0 as i32 + ox;
        let new_pos_y = pos.1 as i32 + oy;

        if new_pos_x < 0 || new_pos_x >= grid.len() as i32 {
            continue;
        }
        if new_pos_y < 0 || new_pos_y >= grid[0].len() as i32 {
            continue;
        }

        if grid[new_pos_x as usize][new_pos_y as usize] != prev {
            continue;
        }

        flood_fill(grid, (new_pos_x as usize, new_pos_y as usize), area);

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
