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

    let input = word_input_to_number(input);

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

fn word_input_to_number(input: &str) -> String {

    let mut output = "".to_string();
    

    for i in 0..input.len() {
        
        if char::is_digit(input.chars().nth(i).unwrap(), 10) {
            output.push(input.chars().nth(i).unwrap());  
        }
        if input.len() - i >= 3 {
            if &input[i..i+3] == "one" {
                output += "1";
            }
            if &input[i..i+3] == "two" {
                output += "2";
            }
            if &input[i..i+3] == "six" {
                output += "6";
            }
        }
        if input.len() - i >= 4 {
            if &input[i..i+4] == "four" {
                output += "4";
            }
            if &input[i..i+4] == "five" {
                output += "5";
            }
            if &input[i..i+4] == "nine" {
                output += "9";
            }
        }
        if input.len() - i >= 5 {
            if &input[i..i+5] == "three" {
                output += "3";
            }
            if &input[i..i+5] == "seven" {
                output += "7";
            }
            if &input[i..i+5] == "eight" {
                output += "8";
            }
        }

    }
    return output;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_parse_line_simple_outer() {
        let result = parse_line("1abc2");
        assert_eq!(result,12);
    }

    #[test]
    fn part_1_parse_line_simple_inner() {
        let result = parse_line("pqr3stu8vwx");
        assert_eq!(result, 38);
    }
    
    #[test]
    fn part_1_parse_line_many_inner() {
        let result = parse_line("a1b2c3d4e5f");
        assert_eq!(result, 15);
    }

    #[test]
    fn part_1_parse_line_one_inner() {
        let result = parse_line("treb7uchet");
        assert_eq!(result, 77);
    }

    #[test]
    fn part_1_parse_example() {
        let result = parse(r"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet");
        assert_eq!(result, 142);
    }

    #[test]
    fn part_2_parse_line_simple_words() {
        let result = parse_line("two1nine");
        assert_eq!(result, 29);
    }

    #[test]
    fn part_2_parse_line_all_words() {
        let result = parse_line("eightwothree");
        assert_eq!(result, 83);
    }

    #[test]
    fn part_2_parse_line_word_inner() {
        let result = parse_line("abcone2threexyz");
        assert_eq!(result, 13);
    }

    #[test]
    fn part_2_parse_line_twone_as_two() {
        let result = parse_line("xtwone3four");
        assert_eq!(result, 24); 
    }

    #[test]
    fn part_2_parse_line_use_numbers_still() {
        let result = parse_line("4nineeightseven2");
        assert_eq!(result, 42);
    }

    #[test]
    fn part_2_parse_line_word_and_numbers() {
        let result = parse_line("zoneight234");
        assert_eq!(result, 14);
    }

    #[test]
    fn part_2_parse_number_and_word() {
        let result = parse_line("7pqrstsixteen");
        assert_eq!(result, 76);
    }

    #[test]
    fn part_2_parse_example() {
        let result = parse(r"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
");
        assert_eq!(result, 281);
    }

}
