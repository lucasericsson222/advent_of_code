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
            if j as i32 - 1 < 0 || !lines[i].chars().nth(j - 1 as usize).expect("This subtraction should work because the or short-circuits").is_digit(10) {
                if lines[i][j..j+1]
                    .chars()
                    .nth(0)
                    .expect("This index should exist")
                    .is_digit(10) {
                    
                    sum += calc_is_gear(i, j, &lines);

                }
            }
        }
    }

    sum 
}

fn calc_is_gear(i: usize, j: usize, lines: &Vec<&str>) -> i32 {
    
    let num = get_number(i, j, lines);

    if is_neighboring_symbol(i, j, lines) {
        return num;
    }

    if num / 10 != 0 {
        if is_neighboring_symbol(i, j+1, lines) {
            return num;
        }
    }

    if num / 100 != 0 {
        if is_neighboring_symbol(i, j+2, lines) {
            return num;
        }
    }

    return 0;
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

fn is_neighboring_symbol(i: usize, j: usize, lines: &Vec<&str>) -> bool {
    for dy in -1..=1 {
        for dx in -1..=1 {
            if i as i32 + dy >= 0 {
                let iindex = i as i32 + dy;
                let jindex = j as i32 + dx;
                if let Some(line) = lines.get(iindex as usize) {
                    if let Some(value) = line.get(jindex as usize .. (jindex + 1) as usize) {
                        if *value != *"." && !value
                            .chars()
                            .nth(0)
                            .expect("The zeroth index should exist")
                            .is_digit(10) {
                            
                            return true;

                        }
                    }
                }
            }
        }
    }
    return false; 
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
        assert_eq!(result, 4361);
    }

    #[test]
    fn get_number_example() {
        let result = get_number(0, 0, &vec!["234"]); 
        assert_eq!(result, 234);
        
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
        assert_eq!(result, 413);
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
        assert_eq!(result, 925);
    }

    #[test]
    fn is_neighboring_symbol_check() {
        let input = r"12.......*..
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

        assert_eq!(is_neighboring_symbol(9, 0, &input), true);
    }

    #[test]
    fn is_neighboring_symbol_check_2() {
        let input = r"12.......*..
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

        assert_eq!(is_neighboring_symbol(9, 2, &input), true);
        
        assert_eq!(is_neighboring_symbol(7, 5, &input), true);
        assert_eq!(is_neighboring_symbol(0, 0, &input), true);
        assert_eq!(is_neighboring_symbol(1, 10, &input), true);
        assert_eq!(is_neighboring_symbol(5, 0, &input), false);
        assert_eq!(is_neighboring_symbol(5, 1, &input), true);
        assert_eq!(is_neighboring_symbol(6, 2, &input), false);
        assert_eq!(is_neighboring_symbol(7, 0, &input), false);
    }

    #[test]
    fn calc_is_gear_check() {
        let input = r"12.......*..
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

        assert_eq!(calc_is_gear(7, 0, &input), 0);
        assert_eq!(calc_is_gear(0, 0, &input), 12);
        assert_eq!(calc_is_gear(0, 1, &input), 2);
    }
}
