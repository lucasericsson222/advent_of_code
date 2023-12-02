fn main() {
    let input = include_str!("./input1.txt");
    let output = parse(input);
    dbg!(output);
}

fn parse(input: &str) -> i32 {
    let mut sum = 0;
    
    for line in input.lines() {
        sum += parse_line(line);
    }
    return sum;
}

fn parse_line(input: &str) -> i32 {
    let mut max_red = 0;
    let mut max_blue = 0;
    let mut max_green = 0;

    let input = input.replace("Game ", "");
    let colon_split: Vec<&str> = input.split(":").collect();
    let rest = colon_split[1];

    for group in rest.split([';', ',']) {
        let space_split: Vec<&str> = group.split_whitespace().collect();
        assert_eq!(space_split.len(), 2);

        let value: i32 = str::parse(space_split[0]).expect("This should be a number count of blocks here");
        let color = space_split[1];

        if color == "blue" {
            if value > max_blue {
                max_blue = value;
            }
        }
        if color == "red" {
            if value > max_red {
                max_red = value;
            }
        }
        if color == "green" {
            if value > max_green {
                max_green = value;
            }
        }
    }

    return max_red * max_blue * max_green;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example() {

        let result = parse(r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        assert_eq!(result, 2286);
    }
}
