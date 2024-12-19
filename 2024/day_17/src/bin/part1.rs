fn main() {
    let input = include_str!("./input1.txt");
    let output = parse(input);
    dbg!(output);
}

fn parse(input: &str) {
    let (a, b) = input.split_once("\n\n").unwrap();

    let mut registers = vec![];
    for item in a.lines() {
        let val = item.split_once(": ").unwrap().1.parse::<usize>().unwrap();
        registers.push(val);
    }

    let inst: Vec<usize> = b.split_once(": ").unwrap().1.split(",").map(|x| x.parse::<usize>().unwrap()).collect();
    
    let combo = |x, registers: &Vec<usize>| -> usize {
        match x {
        0 => 0_usize,
        1 => 1_usize,
        2 => 2_usize,
        3 => 3_usize,
        4 => registers[0],
        5 => registers[1],
        6 => registers[2],
        7 => panic!("7"),
        _ => panic!("undefined combo"),
    }};

    let mut i = 0;
    let mut output = vec![];
    let mut t = 0;
    while i < inst.len() {
        let mut out = registers.clone(); 
        match inst[i] {
            0 => {
                out[0] = (registers[0] as f64 / 2_usize.pow(combo(inst[i+1], &registers) as u32) as f64) as usize;
                //println!("adv A:{} cn:{} -> A", registers[0], combo(inst[i+1], &registers))
            },
            1 => {
                out[1] = registers[1] ^ inst[i+1];
                //println!("bxl B:{} n:{} -> B", registers[1], inst[i+1]);
            },
            2 => {
                out[1] = combo(inst[i+1], &registers) % 8;
                //println!("bst cn:{} -> B", inst[i+1]);
            },
            3 => {
                //println!("jnz A:{}, n:{} -> i", registers[0], inst[i+1]);
                if registers[0] != 0 {
                    i = 2*inst[i+1];
                    continue;
                } 
            },
            4 => {
                out[1] = registers[1] ^ registers[2];
                //println!("bxc B:{} C:{}", registers[1], registers[2]);
            },
            5 => {
                output.push(combo(inst[i + 1], &registers) % 8);
                //println!("out cn:{} -> {}", inst[i + 1], combo(inst[i + 1], &registers));
            },
            6 => {
                out[1] = (registers[0] as f64 / 2_usize.pow(combo(inst[i+1], &registers) as u32) as f64) as usize;
                //println!("bdv A:{} cn:{} -> B", registers[0], inst[i+1]);
            },
            7 => {
                out[2] = (registers[0] as f64 / 2_usize.pow(combo(inst[i+1], &registers) as u32) as f64) as usize;
                //println!("bdv A:{} cn:{} -> C", registers[0], inst[i+1]);

            },
            _ => panic!("unknown instruction")
        };
        registers = out;
        i += 2;
        t += 1;
    }
    for (i, item) in output.iter().enumerate() {
        if i == output.len() - 1 {
            print!("{}", item);
        } else {
            print!("{},", item);
        }
    }
    println!();
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
