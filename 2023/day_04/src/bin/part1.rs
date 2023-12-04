fn main() {
    let input = include_str!("./input1.txt");
    let output = parse(input);
    dbg!(output);
}

fn parse(input: &str) -> i32 {
    let lines = input.lines();
    let mut sum = 0;
    for line in lines {
        let mut value = 0;
        let numbers_on_card = line
            .split([':', '|'])
            .collect::<Vec<&str>>()[1]
            .split_whitespace()
            .map(|x| x.to_string().parse::<i32>().unwrap());
        let winning_numbers = line
            .split([':', '|'])
            .collect::<Vec<&str>>()[2]
            .split_whitespace()
            .map(|x| x.to_string().parse::<i32>().unwrap()).collect::<Vec<i32>>();
        for card in numbers_on_card {
            if winning_numbers.contains(&card) {
                if value == 0 {
                    value = 1;
                } else {
                    value *= 2;
                }
            }
        }
        sum += value;
    }

    sum 
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn parse_example() {
        let result = parse(r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
");
        assert_eq!(result, 13);
    }
}
