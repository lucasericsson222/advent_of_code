fn main() {
    let input = include_str!("./input1.txt");
    let output = parse(input);
    dbg!(output);
}

fn parse(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut sum = 0;

    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            let value = lines[i][j..j+1]
                    .chars()
                    .nth(0)
                    .expect("This index should exist");
            if value == '*' {
                let neighbors = get_number_neighbors(i, j, &lines);
                if neighbors.len() == 2 {
                    sum += neighbors[0] * neighbors[1];
                }
            }
        }
    }

    sum 
}

fn get_number_neighbors(i: usize, j: usize, lines: &Vec<&str>) -> Vec<i32> {
    let mut neighbors: Vec<i32> = vec![];
    let mut previous_locations: Vec<(usize, usize)> = vec![];
    for dy in -1..=1 {
        for dx in -1..=1 {
            if i as i32 + dy >= 0 {
                let iindex = i as i32 + dy;
                let jindex = j as i32 + dx;
                if let Some(line) = lines.get(iindex as usize) {
                    if let Some(value) = line.get(jindex as usize .. (jindex + 1) as usize) {
                        if *value != *"." && value
                            .chars()
                            .nth(0)
                            .expect("The zeroth index should exist")
                            .is_digit(10) {
                            
                            let (number_i, number_j) = get_left_most_digit(iindex as usize, jindex as usize, lines);
                            let number = get_number(number_i, number_j, lines); 
                            if !previous_locations.contains(&(number_i, number_j)) {
                                neighbors.push(number);
                                previous_locations.push((number_i, number_j));
                            }
                        }
                    }
                }
            }
        }
    }

    return neighbors;
}

fn get_left_most_digit(i: usize, j: usize, lines: &Vec<&str>) -> (usize, usize) {
    let mut index = j as i32;
    while index >= 0 && lines[i][(index) as usize ..(index+1) as usize].chars().nth(0).unwrap().is_digit(10) {
        index -= 1;
    }

    (i, (index + 1) as usize)
}

fn get_number(i: usize, j: usize, lines: &Vec<&str>) -> i32 {
    let mut num = lines[i].chars().nth(j).expect("This index should exist").to_digit(10).expect("This should be a digit");
    
    if let Some(right_value) = lines[i].chars().nth(j+1) {
        if right_value.is_digit(10) {
            num *= 10;
            num += right_value.to_digit(10).unwrap();

            if let Some(right_right_value) = lines[i].chars().nth(j+2) {
                if right_right_value.is_digit(10) {
                    num *= 10;
                    num += right_right_value.to_digit(10).unwrap();
                }
            }
        }
    }


    return num as i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example() {
        let result = parse(r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
");
        assert_eq!(result, 467835);
    }

    #[test]
    fn parse_different() {
        let result = parse(r"12.......*..
+.........34
.......-12..
..78........
..*....60...
78..........
.......23...
....90*12...
............
2.2......12.
.*.........*
1.1.......56");
        assert_eq!(result, 6756);
    }

    #[test]
    fn parse_even_better() {
        let result = parse(r"12.......*..
+.........34
.......-12..
..78........
..*....60...
78.........9
.5.....23..$
8...90*12...
............
2.2......12.
.*.........*
1.1..503+.56");
        assert_eq!(result, 6756);
    }

    #[test]
    fn get_right_most_digit_check() {
        let result = r"12.......*..
+.........34
.......-12..
..78........
..*....60...
78.........9
.5.....23..$
8...90*12...
............
2.2......12.
.*.........*
1.1..503+.56".lines().collect();
        assert_eq!(get_left_most_digit(0, 1, &result), (0, 0));
        assert_eq!(get_left_most_digit(11, 7, &result), (11, 5));
        assert_eq!(get_left_most_digit(5, 11, &result), (5, 11));
    }

}
