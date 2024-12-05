fn main() {
    let input = include_str!("./input1.txt");
    let output = parse(input);
    dbg!(output);
}

fn parse(input: &str) -> i32 {
    let mut sum = 0;

    for (id, ch) in input.chars().into_iter().enumerate() {
        if ch == '(' {
            sum += 1;
        } 
        if ch == ')' {
            sum -= 1;
        }
        
        if sum < 0 {
            return (id + 1) as i32;
        }
        
    }

    panic!();
}
