use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = parse(input);
    dbg!(output);
}

fn parse(input: &str) -> i32 {
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
    let mut not_found_zzz = true;
    let mut current_location = "AAA";
    let mut num_steps = 0;
    while not_found_zzz {
        for ch in inst.chars() {

            if current_location == "ZZZ" {
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

    return num_steps;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let result = parse(r"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
");
        assert_eq!(result, 2);
    }

    #[test]
    fn test_loop_parse() {
        let result = parse(r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
");
        assert_eq!(result, 6);
    }

}
