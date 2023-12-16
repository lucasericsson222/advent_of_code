fn main() {
    let input = include_str!("./input.txt");
    let output = parse(input);
    dbg!(output);
}

fn parse(input: &str) -> usize {
    let mut map: Vec<Vec<&str>> = vec![vec![]; 256];
    let instructions = input.replace("\n", "");

    for instruction in instructions.split(",") {
        if instruction.contains('-') {
            let label = instruction.split('-').nth(0).unwrap();
            let mut new_vec = vec![];
            for item in &map[hash(label)] {
                if !item.contains(label) {
                    new_vec.push(*item);  
                }
            }
            map[hash(label)] = new_vec;
        }
        if instruction.contains('=') {
            let label = instruction.split('=').nth(0).unwrap();
            let mut found = false;
            for item in &mut map[hash(label)] {
                if item.contains(label) {
                    *item = instruction;
                    found = true;
                }
            }
            if !found {
                map[hash(label)].push(instruction);
            }
        }
    }

    map
        .iter()
        .enumerate()
        .map(
            |(hash_id, vector)|
            vector
                .iter()
                .enumerate()
                .map(
                    |(position, item)|
                    (hash_id + 1) * (position + 1) * item.chars().last().unwrap().to_digit(10).unwrap() as usize
                ).sum::<usize>()
        ).sum()
}

fn hash(instruction: &str) -> usize {
    instruction
        .chars()
        .map(|ch| (ch as u8))
        .fold(0, |left, right| ((left + right as usize) * 17) % 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_first_instruction() {
        let result = parse(r"rn=1");
        assert_eq!(result, 1);
    }

    #[test]
    fn parse_example() {
        let result = parse(r"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");
        assert_eq!(result, 145);
    }

}
