use std::collections::HashSet;

fn main() {
    let input = include_str!("./input1.txt");
    let output = parse(input);
    dbg!(output);
}

fn parse(input: &str) -> usize {
    let mut grid = vec![];
    let mut row1 = vec![];
    let mut row2 = vec![];

    for line in input.lines() {
        let mut row = vec!['.'];
        for ch in line.chars() {
            row.push(ch);
        }
        row.push('.');
        grid.push(row);
    }
    for i in 0..grid[0].len() {
        row1.push('.'); 
        row2.push('.');
    }
    grid.insert(0, row1);
    grid.push(row2);


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

    return cur_area.len() * perimeter(&cur_area, &grid);
}

fn perimeter(area: &HashSet<(usize, usize)>, grid: &Vec<Vec<char>>) -> usize {

    let mut hashsets = vec![HashSet::new(), HashSet::new(), HashSet::new(), HashSet::new()];
    let mut counts = vec![0, 0, 0, 0];
    for pos in area {
        for (i, (ox, oy)) in vec![(-1, 0), (1,0), (0, -1), (0, 1)].iter().enumerate() {
            let new_pos_x = pos.0 as i32 + ox;
            let new_pos_y = pos.1 as i32 + oy;
            
            if new_pos_x < 0 || new_pos_y < 0 || !area.contains(&(new_pos_x as usize, new_pos_y as usize)) {
                hashsets[i].insert((pos.0, pos.1));
            } 
        }
    }

    for i in 0..4 {
        for pos in &hashsets[i] {
            let val = adjacent_poss(&pos, i);
            if val.len() == 1 {
                counts[i] += 1;
            } else {
                if !hashsets[i].contains(&val[1]) {
                    println!("{:?}, {:?}, {}", pos, val[1], i);
                    counts[i] += 1;
                }
            }
        }
    }

    return dbg!(counts.iter().sum());
}

fn adjacent_poss(pos: &(usize, usize), i: usize) -> Vec<(usize, usize)> {
    let mut out = vec![];

    if i == 2 || i == 3 {
        if pos.0 != 0 {
            out.push((pos.0 - 1, pos.1));
        }
        out.push((pos.0 + 1, pos.1));
    } else {
        if pos.1 != 0 {
            out.push((pos.0, pos.1 - 1));
        }
        out.push((pos.0, pos.1 + 1));
    }

    return out
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
        let result = parse(r"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
");
        assert_eq!(result, 368);
    }

    #[test]
    fn parse_example2() {
        let result = parse(r"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
");
        assert_eq!(result, 236);
    }

}
