use std::mem::swap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = parse(input);
    dbg!(output);
}

#[derive(Clone, Copy)]
struct Val {
    id: i32,
}


fn parse(input: &str) -> i64 {

    let mut vals = vec![];

    for (i, ch) in input.chars().enumerate() {
        let val = ch.to_digit(10).unwrap();
        if i % 2 == 0 {
            for _ in 0..val {
                vals.push(Val {id: (i/2) as i32});
            } 
        } else {
            for _ in 0..val {
                vals.push(Val {id: -1});
            }
        }
    }

    for (i, val) in vals.iter().enumerate() {
        // print!("{}, {}\n", i, val.id);
    }

    for i in (0..vals.len()).rev() {
        if vals[i].id == -1 {
            continue;
        }
        for j in 0..i {
            if vals[j].id == -1 {
                vals[j] = vals[i];
                vals[i] = Val {id: -1};
                break;
            }
        }
    }

    let mut sum: i64 = 0;
    for (i, val) in vals.iter().enumerate() {
        if val.id == -1 {
            continue;
        }
        sum += val.id as i64 * i as i64;
    }

    return sum;
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
