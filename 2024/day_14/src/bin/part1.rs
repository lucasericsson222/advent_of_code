fn main() {
    let input = include_str!("./input1.txt");
    let output = parse(input);
    dbg!(output);
}

fn parse(input: &str) -> usize {
    let mut counts = vec![0; 4];
    let width = 101;
    let height = 103;
    //let width = 11;
    //let height = 7;
    for line in input.lines() {
        let (p, v) = line.split_once(" ").unwrap();
        let x = p.split_once("=").unwrap().1.split_once(",").unwrap().0.parse::<i32>().unwrap();
        let y = p.split_once(",").unwrap().1.parse::<i32>().unwrap();
        let vx = v.split_once("=").unwrap().1.split_once(",").unwrap().0.parse::<i32>().unwrap();
        let vy = v.split_once(",").unwrap().1.parse::<i32>().unwrap();

        let fx = (x + vx * 100).rem_euclid(width);
        let fy = (y + vy * 100).rem_euclid(height);

        let midx = width / 2;
        let midy = height / 2;

        if fx == midx {
            continue;
        }
        if fy == midy {
            continue;
        }

        if fx < midx {
            if fy < midy {
                counts[0] += 1;
            } else {
                counts[1] += 1;
            }
        } else {
            if fy < midy {
                counts[2] += 1;
            } else {
                counts[3] += 1;
            }
        }
    }

    return counts.iter().product();
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
