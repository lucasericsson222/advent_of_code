use core::time;
use std::{collections::HashSet, hash::Hash, thread};

fn main() {
    let input = include_str!("./input1.txt");
    let output = parse(input);
    dbg!(output);
}

fn parse(input: &str) -> usize {
    let mut sec = 0;
    let mut max_val = 0;
    loop {
        let cur = run_steps(input, sec);
        //display(cur); 
        let cur_val = count_neighbors(&cur);
        if cur_val >= max_val {
            max_val = cur_val;
            println!("{}", sec);
            println!("{}", cur_val);
            display(&cur);
        }
        sec += 1;
    }
    return 0;
}

fn count_neighbors(cur: &HashSet<(i32, i32)>) -> i32 {

    let mut count = 0;

    for pos in cur {
        for offset in vec![(1,0), (0, 1), (-1, 0), (0, -1)] {
            if cur.contains(&(pos.0 + offset.0, pos.1 + offset.1)) {
                count += 1;
            }
        }
    }
    count
}

fn display(cur: &HashSet<(i32, i32)>) {
    let width = 101;
    let height = 103;

    for i in 0..width {
        for j in 0..height {
            if cur.contains(&(i,j)) {
                print!("#");
                continue;
            }
            print!(" ");
        }
        println!();
    } 
    println!();
}

fn run_steps(input: &str, sec: i32) -> HashSet<(i32, i32)> {
    let width = 101;
    let height = 103;
    //let width = 11;
    //let height = 7;
    let mut res = HashSet::new();
    for line in input.lines() {
        let (p, v) = line.split_once(" ").unwrap();
        let x = p.split_once("=").unwrap().1.split_once(",").unwrap().0.parse::<i32>().unwrap();
        let y = p.split_once(",").unwrap().1.parse::<i32>().unwrap();
        let vx = v.split_once("=").unwrap().1.split_once(",").unwrap().0.parse::<i32>().unwrap();
        let vy = v.split_once(",").unwrap().1.parse::<i32>().unwrap();

        let fx = (x + vx * sec).rem_euclid(width);
        let fy = (y + vy * sec).rem_euclid(height);
        res.insert((fx, fy));
    }
    res
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
