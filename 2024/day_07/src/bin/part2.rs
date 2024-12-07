fn main() {
    let input = include_str!("./input1.txt");
    let output = parse(input);
    dbg!(output);
}

fn parse(input: &str) -> i64 {
    let mut sum: i64 = 0;
    for line in input.lines() {
        let (test_value, calibration_equation) = line.split_once(":").unwrap();
        let test_value = test_value.parse::<i64>().unwrap();
        let values: Vec<i64> = calibration_equation.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect();

        if can_equal(test_value, values[0], values.split_first().unwrap().1) {
            sum += test_value as i64;
        }

    }
    return sum;
}

fn can_equal(goal: i64, cur_value: i64, remaining_values: &[i64]) -> bool {
    if remaining_values.len() == 0 {
        goal==cur_value
    } else {
        can_equal(
            goal, 
            cur_value + remaining_values[0], 
            remaining_values.split_first().unwrap().1) 
        || can_equal(
            goal, 
            cur_value * remaining_values[0], 
            remaining_values.split_first().unwrap().1)
        || can_equal(
            goal, 
            (cur_value.to_string() + &remaining_values[0].to_string()).parse::<i64>().unwrap(),
            remaining_values.split_first().unwrap().1)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example() {
        let result = parse(r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
");
        assert_eq!(result, 18);
    }

}
