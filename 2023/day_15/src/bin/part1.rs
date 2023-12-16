fn main() {
    let input = include_str!("./input.txt");
    let output = parse(input);
    dbg!(output);
}

fn parse(input: &str) -> usize {
    let sum = input
        .replace("\n", "")
        .split(',')
        .map(
            |instruction| 
            instruction
                .chars()
                .map(|ch| (ch as u8))
                .fold(0, |left, right| ((left + right as usize) * 17) % 256)
        )
        .sum();

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn parse_example() {
        let result = parse(r"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");

        assert_eq!(result, 1320);
    }

    #[test]
    fn parse_line_1() {
        let result = parse(r"rn=1");
        assert_eq!(result, 30);
    }

    #[test]
    fn parse_line_2() {
        let result = parse(r"cm-");
        assert_eq!(result, 253);
    }

    #[test]
    fn parse_hash() {
        let result = parse(r"HASH");
        assert_eq!(result, 52);
    }
}
