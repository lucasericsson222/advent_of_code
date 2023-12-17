use std::{collections::BinaryHeap, cmp::Reverse, char::from_digit};

fn main() {
    let input = include_str!("./input.txt");
    let output = parse(input);
    dbg!(output);
}

fn parse(input: &str) -> usize {

    let mut grid = vec![];

    for line in input.lines() {
        let mut row = vec![];
        for char in line.chars() {
            row.push(char.to_digit(10).unwrap() as usize);
        }
        grid.push(row);
    }

    return dijkstra(&grid);
}


#[derive(PartialEq, Eq, Clone)]
struct Node {
    row: usize,
    col: usize,
    dir: usize,
    dist: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.dist.cmp(&other.dist);
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return self.dist.partial_cmp(&other.dist);
    }
}

fn dijkstra(weights: &Vec<Vec<usize>>) -> usize {

    let mut heap = BinaryHeap::new();

    let mut dist = vec![vec![vec![usize::MAX;4]; weights[0].len()]; weights.len()];
    let mut prev = vec![vec![vec![None;4]; weights[0].len()]; weights.len()];
    // up, down, left, right
    dist[0][0] = vec![0;4];
    
    for i in 0..weights.len() {
        for j in 0..weights[0].len() {
            for dir in 0..4 {
                heap.push(Reverse(Node {row: i, col: j, dir, dist: dist[i][j][dir]}));
            }
        }
    }

    while let Some(Reverse(current)) = heap.pop() {
        if current.dist == usize::MAX {
            continue;
        }
        for neighbor in neighbors(&current, &weights) {
            if dist[neighbor.row][neighbor.col][neighbor.dir] > neighbor.dist {
                dist[neighbor.row][neighbor.col][neighbor.dir] = neighbor.dist;
                prev[neighbor.row][neighbor.col][neighbor.dir] = Some(current.clone());
                heap.push(Reverse(neighbor));
            }
        }
    }
    
    let mut out = vec![vec!['.';weights[0].len()]; weights.len()];
    let mut node = &prev[prev.len()-1][prev[0].len()-1][3];
    let mut i = 0;
    while *node != None {
        if let Some(mynode) = node {
            out[mynode.row][mynode.col] = from_digit(i % 10, 10).unwrap();
            node = &prev[mynode.row][mynode.col][mynode.dir];
        }
        i += 1;
    }
    let max_row = out.len() - 1;
    let max_col = out[0].len() - 1;
    out[max_row][max_col] = '#';

    print_grid(&out);
    println!("{}", dist[dist.len() - 1][dist[0].len() -1][3]);

    let mut out = vec![vec!['.';weights[0].len()]; weights.len()];
    let mut node = &prev[prev.len()-1][prev[0].len()-1][1];
    let mut i = 0;
    while *node != None {
        if let Some(mynode) = node {
            out[mynode.row][mynode.col] = from_digit(i % 10, 10).unwrap();
            node = &prev[mynode.row][mynode.col][mynode.dir];
        }
        i += 1;
    }
    let max_row = out.len() - 1;
    let max_col = out[0].len() - 1;
    out[max_row][max_col] = '#';

    print_grid(&out);
    println!("{}", dist[dist.len() - 1][dist[0].len() -1][1]);

    return std::cmp::min(dist.last().unwrap().last().unwrap()[1], dist.last().unwrap().last().unwrap()[3]);
}


fn neighbors(current: &Node, weights: &Vec<Vec<usize>>) -> Vec<Node> {
    let mut result = vec![];

    for dir in 0..4 {
        if dir == current.dir {
            continue;
        }
        if is_reverse(dir, current.dir) {
            continue;
        }
        let mut dist_sum = current.dist;
        for jump_length in 1..=10 {
            let (mut dir_row, mut dir_col) = dir_to_tuple(dir);
            dir_row *= jump_length; 
            dir_col *= jump_length;
            let row_index = current.row as i32 + dir_row;
            let col_index = current.col as i32 + dir_col;
            // bounds checking of array
            if row_index < 0 || row_index >= weights.len() as i32 { 
                break;
            }
            if col_index < 0 || col_index >= weights[0].len() as i32 {
                break;
            }
            dist_sum += weights[row_index as usize][col_index as usize];

            if jump_length < 4 {
                continue;
            }
            
            result.push(
                Node {
                    row: row_index as usize,
                    col: col_index as usize,
                    dir,
                    dist: dist_sum,
                }
            );
        }
    }

    return result;
}

fn is_reverse(dir: usize, other_dir: usize) -> bool {
    match dir {
        0 => other_dir == 1,
        1 => other_dir == 0,
        2 => other_dir == 3,
        3 => other_dir == 2,
        _ => panic!("dir isn't 0-4"),
    }
}

fn dir_to_tuple(dir: usize) -> (i32, i32) {
    match dir {
        0 => (-1, 0), // up is -1 on row
        1 => (1, 0), // down is 1 on row
        2 => (0, -1), // left is -1 on col
        3 => (0, 1), // right is 1 on col
        _ => panic!("dir isn't 0-4"),
    }
}


fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        for ch in row {
            print!("{}", ch);
        }
        println!();
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn parse_example() {
        let result = parse(r"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
");
        assert_eq!(result, 94);
    }
}
