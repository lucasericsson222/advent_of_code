fn main() {
    let input = include_str!("./input1.txt");
    let output = parse(input);
    dbg!(output);
}

fn parse(input: &str) -> i32 {
    let mut sum = 0;

    for line in input.lines() {
        let mut safe = true;
        let mut previous: Option<i32> = None;
        let mut increasing: Option<bool> = None;
        for item in line.split_whitespace() {
            let el: i32 = item.parse::<i32>().unwrap();
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
            previous = Some(el);
        }

        if safe {
            sum += 1;
        }
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
