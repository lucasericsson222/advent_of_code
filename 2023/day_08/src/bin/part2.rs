use std::collections::HashMap;
use num_integer::Integer;

fn main() {
    let input = include_str!("./input.txt");
    let output = parse(input);
    dbg!(output);
}

fn parse(input: &str) -> i64 {
    let mysplit: Vec<&str> = input.split("\n\n").collect();
    let inst = mysplit[0];
    let map_keys = mysplit[1];
    let mut directions = HashMap::new();
    for key_str in map_keys.lines() {
        let key_split: Vec<&str> = key_str.split_whitespace().collect();
        let source = key_split[0]; 
        let left_dest = &key_split[2][1..4];
        let right_dest = &key_split[3][0..3];
        directions.insert(source, (left_dest, right_dest));
    }
    let mut lengths_of_cycles: Vec<i64> = vec![];
    for (start_location, _) in &directions {
        if start_location.chars().nth(2).unwrap() == 'A'  {
            let mut not_found_zzz = true;
            let mut current_location = *start_location;
            let mut num_steps = 0;
            while not_found_zzz {
                for ch in inst.chars() {
                    
                    if current_location.chars().nth(2).unwrap() == 'Z' {
                        not_found_zzz = false;
                        break;
                    }
                    if ch == 'R' {
                        (_, current_location) = *directions
                            .get(current_location).unwrap();
                    }
                    if ch == 'L' {
                        (current_location, _) = *directions
                            .get(current_location).unwrap();
                    }
                    num_steps += 1; 
                }
            }
            lengths_of_cycles.push(num_steps);
        }
    }

    let result = lengths_of_cycles.iter().fold(1, |left, right| left.lcm(right));
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse() {
        let result = parse(r"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
");
        assert_eq!(result,  6);
    }

}
