fn main() {
    let input = include_str!("./input1.txt");
    let output = parse(input);
    dbg!(output);
}

fn parse(input: &str) -> i32 {
    let mut sum = 0;

    for line in input.lines() {

        let line_vec = into_vec(line);

        let mut safe = parse_line(&line_vec);

        for i in 0..line_vec.len() {
            let mut cur_vec = line_vec.clone();
            cur_vec.remove(i);
            let cur_safe = parse_line(&cur_vec);
            if cur_safe == true {
                safe = true;
            }
        }

        if safe {
            sum += 1;
        }
    }

    return sum;
}

fn into_vec(line: &str) -> Vec<i32> {
    let mut line_vec = vec![];
    for ch in line.split_whitespace() {
        line_vec.push(ch.parse::<i32>().unwrap());
    }
    return line_vec;
} 

fn parse_line(line: &Vec<i32>) -> bool {
    let mut safe = true;
    let mut previous: Option<i32> = None;
    let mut increasing: Option<bool> = None;
    for el in line {
        if let Some(val) = previous {
            if !((el - val).abs() >= 1 && (el - val).abs() <= 3) {
                safe = false;
            }
            if increasing == None {
                if el - val >= 0 {
                    increasing = Some(true);
                }
                if el - val <= 0 {
                    increasing = Some(false);
                }
            }
            if let Some(is_increasing) = increasing {
                if el - val >= 0 {
                    if !is_increasing {
                        safe = false;
                    }
                }
                if el - val <= 0 {
                    if is_increasing {
                        safe = false;
                    }
                }

            }
        }
        previous = Some(*el);
    }
    return safe
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line_1() {
        let input = "1 2 7 8 9"; 
        let line_vec = into_vec(input);
        let result = parse_line(&line_vec);
        assert_eq!(result, false);
    }

    #[test]
    fn parse_example() {
        let result = parse(r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9");
        assert_eq!(result, 4);
    }

}
