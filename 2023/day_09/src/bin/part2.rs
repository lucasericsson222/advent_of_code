fn main() {
    let input = include_str!("./input.txt");
    let output = parse(input);
    dbg!(output);
}

fn parse(input: &str) -> i64 {
    let lines = input.lines();
    let mut sum: i64 = 0;
    for line in lines {

        let mut rows: Vec<Vec<i64>> = vec![];
        rows.push(
            line
                .split_whitespace()
                .map(|value| value.parse::<i64>().unwrap())
                .collect()
        );
        let mut current = 0;
        while !is_zero_vec(&rows[current]) {
            let mut new_vec = vec![];
            let previous_row = &rows[current];
            for i in 0..(rows[current].len()-1) {
                new_vec.push(previous_row[i + 1] - previous_row[i]); 
            }
            rows.push(new_vec);
            current += 1;
        }
        let mut temp_sum = 0;
        for row in rows.iter().rev() {
            temp_sum = row.first().unwrap() - temp_sum;
        }
        sum += temp_sum;
    }

    return sum;
}

fn is_zero_vec(myvec: &Vec<i64>) -> bool {
    for value in myvec {
        if *value != 0 {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example() {
        let result = parse(r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
");
        assert_eq!(result, 2);
    }

    #[test]
    fn custom_example() {
        let result = parse(r"-4 -2 0 2 4");
        assert_eq!(result, -6);
    }

    #[test]
    fn line_test() {
        let result = parse(r"9 16 20 21 19 14 6 -5 -19 -36 -56 -79 -105 -134 -166 -201 -239 -280 -324 -371 -421");
        assert_eq!(result, -1);

    }
}
