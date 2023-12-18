fn main() {
    let input = include_str!("./input.txt");
    let output = parse(input);
    dbg!(output);
}

fn parse(input: &str) -> usize {
    let mut sum = 0;
   
    let mut vertices = vec![];
    let mut current_location = (0,0);
    vertices.push(current_location);
    for line in input.lines() {
        let split: Vec<&str> = line.split_whitespace().collect();
        let instruction = split[0];
        let steps: u32 = split[1].parse().unwrap();
        let dir = instruction_to_dir(instruction);

        let offset = mul(dir, steps as i32);

        current_location = add(current_location, offset);
        vertices.push(current_location); 
    }

    vertices.pop();

    let mut perimeter = 0;
    for i in 0..vertices.len() {
        perimeter += length(sub(vertices[i], vertices[(i + 1) % vertices.len()]));
    }
    
    for i in 0..vertices.len() {
        let area;
            area = det(vertices[i], vertices[(i+1) % vertices.len()]);
        sum += area;
    }

    sum /= 2;

    sum = num::abs(sum);
    
    sum += perimeter / 2 + 1;

    return sum as usize;
}

fn sub(vec1: (i32, i32), vec2: (i32, i32)) -> (i32, i32) {
    (vec1.0 - vec2.0, vec1.1 - vec2.1)
}

fn length(vec1: (i32, i32)) -> i32 {
    if vec1.0 == 0 {
        num::abs(vec1.1) 
    } else if vec1.1 == 0 {
        num::abs(vec1.0)
    } else {
        panic!("Non straight vertex differences");
    }
}

fn det(vec1: (i32, i32), vec2: (i32, i32)) -> i32 {
    vec1.0 * vec2.1 - vec2.0 * vec1.1
}

fn mul(vec: (i32, i32), scalar: i32) -> (i32, i32) {
    (vec.0 * scalar, vec.1 * scalar)
}

fn add(location: (i32, i32), dir: (i32, i32)) -> (i32, i32) {
    (location.0 + dir.0, location.1 + dir.1)
}

fn instruction_to_dir(instruction: &str) -> (i32, i32) {
    match instruction {
        "R" => (1, 0),
        "L" => (-1, 0),
        "U" => (0, 1),
        "D" => (0, -1),
        _ => panic!("Invalid instruction"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn parse_example() {
        let result = parse(r"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
");
        assert_eq!(result, 62);
    }

    #[test]
    fn parse_my_rect() {
        let result = parse(r"R 6 (#70c710)
D 5 (#0dc571)
L 6 (#d2c081)
U 5 (#59c680)
");
        assert_eq!(result, 7*6);
    }
}
