fn main() {
    let input = include_str!("./input1.txt");
    let output = parse(input);
    dbg!(output);
}

fn parse(input: &str) -> i32 {
    let mut sum = 0;

    let chunks: Vec<&str> = input.split("\n\n").collect();
    let assertions = chunks[0];
    let lines = chunks[1];

    let mut asserts_groups = vec![];

    for assertion in assertions.lines() {
        let chunks: Vec<&str> = assertion.split("|").collect();
        let a = chunks[0].parse::<i32>().unwrap();
        let b = chunks[1].parse::<i32>().unwrap();
        asserts_groups.push((a, b));
    }

    let get_index = |values: &Vec<i32>, x: i32| values.into_iter().position(|n| *n==x);

    for line in lines.lines() {
        let items: Vec<i32> = line.split(",").map(|x| x.parse::<i32>().unwrap()).collect();

        let mut valid = true;

        for (a,b) in &asserts_groups {
            let index_a = get_index(&items, *a);
            let index_b = get_index(&items, *b);

            if let Some(index_a) = index_a {
                if let Some(index_b) = index_b {
                    if index_a > index_b {
                        valid = false;
                        break;
                    }
                }
            } 
        }

        if valid {
            sum += items[items.len() / 2];
        }
    }

    return sum;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example() {
        let result = parse(r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
");
        assert_eq!(result, 18);
    }

}
