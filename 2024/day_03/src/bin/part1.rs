use regex::Regex;
fn main() {
    let input = include_str!("./input1.txt");
    let output = parse(input);
    dbg!(output);
}

fn parse(input: &str) -> i32 {
    let mut sum = 0;

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    for (full, [num1, num2]) in re.captures_iter(input).map(|caps| caps.extract()) {
        let numa = num1.parse::<i32>().unwrap();
        let numb = num2.parse::<i32>().unwrap();
        sum += numa*numb;
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example() {
        let result = parse(r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9");
        assert_eq!(result, 2);
    }

}
