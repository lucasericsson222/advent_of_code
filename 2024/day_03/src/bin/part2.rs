use regex::Regex;
use regex::CaptureMatches;
fn main() {
    let input = include_str!("./input1.txt");
    let output = parse(input);
    dbg!(output);
}

fn parse(input: &str) -> i32 {
    let mut sum = 0;

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let redo = Regex::new(r"do").unwrap();
    let redont = Regex::new(r"don\'t").unwrap();


    for cap in re.captures_iter(input) {
        let (_full, [num1, num2]) = cap.extract();
        let numa = num1.parse::<i32>().unwrap();
        let numb = num2.parse::<i32>().unwrap();

        let cur_index  = &cap[0].as_ptr();

        let dos = redo.captures_iter(input);
        let donts = redont.captures_iter(input);
        let result = find_min_index(dos, donts, cur_index);

        if result {
            sum += numa*numb;
        }
    }

    return sum;
}

fn find_min_index(dos: CaptureMatches, donts: CaptureMatches, index: &*const u8) -> bool {
    let mut max_do_less_than_index: Option<*const u8> = None; 
    let mut max_dont_less_than_index: Option<*const u8> = None;
    for cap_do in dos {
        let indo = &cap_do[0].as_ptr().clone();

        if indo < index {
            max_do_less_than_index = Some(indo.clone());
        }
    }
    for cap_dont in donts {
        let indont = &cap_dont[0].as_ptr().clone();
        if indont < index {
            max_dont_less_than_index = Some(indont.clone());
        }
    }

    if let Some(max_do) = max_do_less_than_index {
        if let Some(max_dont) = max_dont_less_than_index {
            if max_do > max_dont {
                return true;
            }
            return false;
        }
    }
    return true;
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example() {
        let result = parse(r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        assert_eq!(result, 48);
    }

}
