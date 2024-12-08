use std::collections::{HashMap, HashSet};
use itertools::Itertools;

fn main() {
    let input = include_str!("./input1.txt");
    let output = parse(input);
    dbg!(output);
}

fn parse(input: &str) -> i32 {
    let mut sum = 0;

    let mut char_to_positions: HashMap<char, HashSet<(i32, i32)>> = HashMap::new(); 

    let mut max_i = 0;
    let mut max_j = 0;

    for (i, line) in input.lines().into_iter().enumerate() {
        for (j, ch) in line.chars().into_iter().enumerate() {
            if ch == '.' {
                continue;
            }
            char_to_positions.entry(ch).or_insert(HashSet::new());
            char_to_positions.get_mut(&ch).map(|val| val.insert((i as i32,j as i32)));
            if j > max_j {
                max_j = j;
            }
        }
        if i > max_i {
            max_i = i;
        }
    }
    max_i += 1;
    max_j += 1;

    let max_i = max_i as i32;
    let max_j = max_j as i32;

    let mut locations: HashSet<(i32, i32)> = HashSet::new();

    for (ch, set) in char_to_positions.iter() {
        for (a,b) in set.iter().tuple_combinations() {
            let delta_x = b.0 as i32 - a.0 as i32;   
            let delta_y = b.1 as i32 - a.1 as i32;

            let new_pos_1_x = delta_x + b.0 as i32;
            let new_pos_1_y = delta_y + b.1 as i32;

            let new_pos_2_x = a.0 as i32 - delta_x;
            let new_pos_2_y = a.1 as i32 - delta_y;

            if check_position(new_pos_1_x, new_pos_1_y, max_i, max_j) {
                locations.insert((new_pos_1_x, new_pos_1_y));
            }
            if check_position(new_pos_2_x, new_pos_2_y, max_i, max_j) {
                locations.insert((new_pos_2_x, new_pos_2_y));
            }

        }
    }

    sum = locations.len() as i32;

    return sum;
}

fn check_position(pos_x: i32, pos_y: i32, max_width: i32, max_height: i32) -> bool {
    if pos_x < 0 || pos_x >= max_width {
        return false;
    }
    if pos_y < 0 || pos_y >= max_height {
        return false;
    }
    return true;
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
