use ethnum::{u256, U256};
use memoize::memoize;

fn main() {
    let input = include_str!("./input1.txt");
    let output = parse(input);
    dbg!(output);
}

fn parse(input: &str) -> usize {
    let mut stones = vec![];
    for item in input.split_whitespace() {
        let item = item.parse::<u256>().unwrap();
        stones.push(item);
    }
    stones.iter().map(|stone| blink(*stone, 75-1)).sum()
}

#[memoize]
fn blink(input: u256, j: usize) -> usize {
    let mut stones = vec![input];
    let mut i = 0;
    while i < stones.len() {
        if stones[i] == 0 {
            stones[i] = U256::new(1);
        } else {
            let cur = stones[i].to_string();
            if cur.len() % 2 == 0 {
                let (left, right) = cur.split_at(cur.len()/2);
                stones[i] = right.parse::<u256>().unwrap();
                stones.insert(i, left.parse::<u256>().unwrap());
                i += 1;
            } else {
                stones[i] = stones[i] * 2024;
            }
        }
        i += 1;
    }
    if j > 0 {
        stones.iter().map(|stone| blink(*stone, j - 1)).sum()
    } else {
        return stones.len();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example() {
        let result = parse(r"");
        assert_eq!(result, 18);
    }

}
