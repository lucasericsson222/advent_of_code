use std::{collections::BinaryHeap, cmp::Reverse};

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


#[derive(Hash, Clone)]
struct Node {
    row: usize,
    col: usize,
    dir: (i32, i32),
    dist: usize,
    num_steps: usize,
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

impl Eq for Node {
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        return 
            self.row == other.row &&
            self.col == other.col &&
            self.dir == other.dir &&
            self.dist == other.dist &&
            self.num_steps == other.num_steps;
    } 
}

fn dijkstra(weights: &Vec<Vec<usize>>) -> usize {

    // (row, col, (dir_row, dir_col), num_prev_steps)
    
    let mut to_go_next = BinaryHeap::new(); 

    to_go_next.push( 
        Reverse(
        Node {
            row: 0,
            col: 0,
            dir: (0, 0),
            dist: 0,
            num_steps: 0,
        }
        )
    );

    let mut previous_states = vec![vec![vec![];weights[0].len()]; weights.len()];

    while let Some(Reverse(current_node)) = to_go_next.pop() {
        for neighbor in neighbors(&current_node) {
            let new_num_steps = neighbor.num_steps;
            if neighbor.row >= weights.len() {
                continue;
            }

            if neighbor.col >= weights[0].len() {
                continue;
            }
            
            let dist = current_node.dist;

            let new_dist = dist + weights[neighbor.row][neighbor.col];
            
            if neighbor.row == weights.len() - 1 && 
                neighbor.col == weights[0].len() - 1 {
                    return new_dist;
            }

            let new_node = 
                    Node {
                        row: neighbor.row,
                        col: neighbor.col,
                        dir: neighbor.dir,
                        dist: new_dist,
                        num_steps: new_num_steps,
                    }; 

            if previous_states[neighbor.row][neighbor.col].contains(&new_node) {
                continue;
            } else {
                previous_states[neighbor.row][neighbor.col].push(new_node.clone()); 
            }

            to_go_next.push(
                Reverse(
                    new_node
                )
            );
        }
    }

    return 0;
}

fn neighbors(current: &Node) -> Vec<Node> {
    let mut result = vec![];
    if current.num_steps < 2 {
        if current.row as i32 + current.dir.0 >= 0 {
            if current.col as i32 + current.dir.1 >= 0 {
                result.push(
                    Node {
                        row: (current.row as i32 + current.dir.0) as usize,
                        col: (current.col as i32 + current.dir.1) as usize,
                        dir: current.dir,
                        dist: current.dist,
                        num_steps: current.num_steps + 1,
                    }
                );
            }
        }
    }

    for dir in vec![(0,1),(1,0),(-1,0),(0,-1)] {
        if dir == current.dir {
            continue;
        }
        if current.row as i32 + dir.0 < 0 {
            continue;
        }
        if current.col as i32 + dir.1 < 0 {
            continue;
        }
        result.push(
            Node {
                row: (current.row as i32 + dir.0) as usize,
                col: (current.col as i32 + dir.1) as usize,
                dir,
                dist: current.dist,
                num_steps: 0,
            }
        );
    }

    return result;
}

fn print_grid(grid: &Vec<Vec<(char, Vec<(i32, i32)>)>>) {
    for row in grid {
        for (ch, _) in row {
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
        assert_eq!(result, 102);
    }
}
