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
        let inter = split[2];
        let instruction = &inter[inter.len() -2..inter.len()-1];
        let steps: u64 = u64::from_str_radix(&inter[2..inter.len()-2], 16).unwrap();
        let dir = instruction_to_dir(instruction);

        let offset = mul(dir, steps as i64);

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

fn sub(vec1: (i64, i64), vec2: (i64, i64)) -> (i64, i64) {
    (vec1.0 - vec2.0, vec1.1 - vec2.1)
}

fn length(vec1: (i64, i64)) -> i64 {
    if vec1.0 == 0 {
        num::abs(vec1.1) 
    } else if vec1.1 == 0 {
        num::abs(vec1.0)
    } else {
        panic!("Non straight vertex differences");
    }
}

fn det(vec1: (i64, i64), vec2: (i64, i64)) -> i64 {
    vec1.0 * vec2.1 - vec2.0 * vec1.1
}

fn mul(vec: (i64, i64), scalar: i64) -> (i64, i64) {
    (vec.0 * scalar, vec.1 * scalar)
}

fn add(location: (i64, i64), dir: (i64, i64)) -> (i64, i64) {
    (location.0 + dir.0, location.1 + dir.1)
}

fn instruction_to_dir(instruction: &str) -> (i64, i64) {
    match instruction {
        "0" => (1, 0),
        "2" => (-1, 0),
        "3" => (0, 1),
        "1" => (0, -1),
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
        assert_eq!(result, 952408144115);
    }
}
