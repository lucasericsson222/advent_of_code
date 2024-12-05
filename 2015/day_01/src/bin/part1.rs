fn main() {
    let input = include_str!("./input1.txt");
    let output = parse(input);
    dbg!(output);
}

fn parse(input: &str) -> i32 {
    let mut sum = 0;

    for ch in input.chars() {
        if ch == '(' {
            sum += 1;
        } 
        if ch == ')' {
            sum -= 1;
        }
    }

    return sum;
}
