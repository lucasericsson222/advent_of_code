fn main() {
    let input = include_str!("./input1.txt");
    let output = parse(input);
    dbg!(output);
}

fn parse(input: &str) -> i32 {
    let mut sum = 0;
    
    for line in input.lines() {
        sum += parse_line(line);
    }

    return sum;
}

fn parse_line(input: &str) -> i32 {
    
    let mut right_most: i32 = -1;
    let mut left_most: i32 = -1;


    for char in input.chars() {

        if left_most == -1 {
            if char.is_digit(10) {
                left_most = char.to_digit(10).unwrap() as i32;
            }
        }

        if char.is_digit(10) {
            right_most = char.to_digit(10).unwrap() as i32;
        }
    }

    return left_most * 10 + right_most;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line_simple_outer() {
        let result = parse_line("1abc2");
        assert_eq!(result,12);
    }

    #[test]
    fn parse_line_simple_inner() {
        let result = parse_line("pqr3stu8vwx");
        assert_eq!(result, 38);
    }
    
    #[test]
    fn parse_line_many_inner() {
        let result = parse_line("a1b2c3d4e5f");
        assert_eq!(result, 15);
    }

    #[test]
    fn parse_line_one_inner() {
        let result = parse_line("treb7uchet");
        assert_eq!(result, 77);
    }

    #[test]
    fn parse_example() {
        let result = parse(r"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet");
        assert_eq!(result, 142);
    }

}
