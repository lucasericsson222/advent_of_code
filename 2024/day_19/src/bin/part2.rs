use std::collections::HashMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = parse(input);
    dbg!(output);
}

fn parse(input: &str) -> usize {
    let (a,b) = input.split_once("\n\n").unwrap();
    let patterns: Vec<&str> = a.split(", ").collect();

    let mut memoize = HashMap::new();

    b.lines().map(|x| is_valid(x, &patterns, &mut memoize)).sum()
}

fn is_valid<'a>(line: &'a str, patterns: &Vec<&str>, memoize: &mut HashMap<&'a str, usize>) -> usize {
    if let Some(val) = memoize.get(line) {
        return *val;
    }
    if line.len() == 0 {
        memoize.insert(line, 1);
        return 1;
    }
    let mut count = 0;
    for pattern in patterns {
        if pattern.len() <= line.len() {
            if **pattern == line[0..pattern.len()] {
                count += is_valid(&line[pattern.len()..], patterns, memoize);
            }
        }
    }

    memoize.insert(line, count);
    return count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example() {
        let result = parse(r"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
");
        assert_eq!(result, 1930);
    }

}
