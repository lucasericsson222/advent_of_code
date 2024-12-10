use std::collections::HashSet;

use itertools::Itertools;

fn main() {
    let input = include_str!("./input1.txt");
    let output = parse(input);
    dbg!(output);
}

fn parse(input: &str) -> i64 {
    let sum: i64;
    
    let mut grid: Vec<Vec<i32>> = vec![]; 

    for line in input.lines() {
        let mut row = vec![];
        for ch in line.chars() {
            row.push(ch.to_digit(10).unwrap() as i32);
        }
        grid.push(row);
    }

    sum = (0..grid.len()).cartesian_product(0..grid[0].len())
        .filter(|(i,j)| grid[*i][*j] == 0)
        .map(|x| count_trailheads(x, &grid, 0).len())
        .sum::<usize>() as i64;

    return sum;
}

fn count_trailheads(pos: (usize, usize), grid: &Vec<Vec<i32>>, cur_val: i32) -> HashSet<(usize, usize)> {
    let (x, y) = pos;
    if grid[x][y] == 9 && cur_val == 9 {
        let mut out = HashSet::new();
        out.insert(pos);
        return out;
    }
    let mut out = HashSet::new();
    for (ox, oy) in vec![(1,0), (0,1), (0, -1), (-1, 0)] {
        let new_x = x as i32 + ox;
        let new_y = y as i32 + oy;
        if new_x < 0 || new_x >= grid.len() as i32 {
            continue;
        }
        if new_y < 0 || new_y >= grid[0].len() as i32 {
            continue;
        }
        if grid[new_x as usize][new_y as usize] == cur_val + 1 {
            let hash = count_trailheads((new_x as usize, new_y as usize), grid, cur_val + 1);
            out.extend(hash); 
        }
    }
    return out;   
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example() {
        let result = parse(r"");
        assert_eq!(result, 18);
    }

}
