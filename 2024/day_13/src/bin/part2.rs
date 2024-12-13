fn main() {
    let input = include_str!("./input1.txt");
    let output = parse(input);
    dbg!(output);
}

fn parse(input: &str) -> usize {
    input.split("\n\n").filter_map(
        |x| {
            let lines: Vec<&str> = x.lines().collect();            
            let v_a = parse_button(lines[0]);
            let v_b = parse_button(lines[1]);
            let (n_x, n_y)= parse_needed(lines[2]);

            let mut n_x = n_x as i64;
            let mut n_y = n_y as i64;
            n_x += 10000000000000;
            n_y += 10000000000000;

            let sol = solve((v_a.0 as i64, v_a.1 as i64), (v_b.0 as i64, v_b.1 as i64), (n_x, n_y));

            if let Some(val) = sol {
                return Some(val.0 as usize * 3 + val.1 as usize);
            }

            return None;
        }).sum()
}

fn solve(a1: (i64, i64), b1: (i64, i64), s: (i64, i64)) -> Option<(i64, i64)> {

    let a = a1.0;
    let b = b1.0;
    let c = a1.1;
    let d = b1.1;
    let e = s.0;
    let f = s.1;

    let det = a*d - b*c;

    if det == 0 {
        return None; 
    } 

    let f1 = e * d - b * f;
    let f2 = a * f - c * e;

    if f1 % det != 0 {
        return None;
    }
    if f2 % det != 0 {
        return None;
    }

    return Some((f1 / det, f2 / det));
}

fn parse_button(input: &str) -> (i32, i32) {
    let num_1 = input.split_once("+").unwrap().1.split_once(",").unwrap().0.parse::<i32>().unwrap();
    let num_2 = input.split_once(",").unwrap().1.split_once("+").unwrap().1.parse::<i32>().unwrap();
    return (num_1, num_2);
}

fn parse_needed(input: &str) -> (i32, i32) {
    let num_1 = input.split_once("=").unwrap().1.split_once(",").unwrap().0.parse::<i32>().unwrap();
    let num_2 = input.split_once(",").unwrap().1.split_once("=").unwrap().1.parse::<i32>().unwrap();
    return (num_1, num_2);
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
