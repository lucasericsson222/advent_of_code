use itertools::Itertools;

fn main() {
    let input = include_str!("./input1.txt");
    let output = parse(input);
    dbg!(output);
}

fn parse(input: &str) -> usize {
    input.split("\n\n").filter_map(
        |x| {
            let lines: Vec<&str> = x.lines().collect();            
            let (m_x_a, m_y_a) = parse_button(lines[0]);
            let (m_x_b, m_y_b) = parse_button(lines[1]);
            let (n_x, n_y)= parse_needed(lines[2]);
            

            let val: usize = (0..=100).cartesian_product(0..=100).filter_map(|(i,j)| {

                if i * m_x_a + j * m_x_b == n_x && i * m_y_a + j * m_y_b == n_y {

                    println!("{}, {}", i, j);
                    return Some((i, j));
                }
                return None;
            }).map(|(a, b)| a as usize * 3 + b as usize).take(1).sum::<usize>();


            if val == 0 {
                return None;
            }

            return Some(val);
        }).sum()
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
