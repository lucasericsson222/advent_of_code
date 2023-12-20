use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = parse(input);
    dbg!(output);
}

struct Rule<'a> {
    name: &'a str,
    maps: Vec<Mapping<'a>>, 
}
#[derive(Debug)]
struct Mapping<'a> {
    catch_all: bool,
    is_lt: bool,
    var: &'a str,
    num: u64,
    new_name: &'a str,
}

#[derive(Clone, Debug)]
struct Point<'a> {
    tag: &'a str,
    x: (u64, u64),
    m: (u64, u64),
    a: (u64, u64),
    s: (u64, u64),
}

fn parse(input: &str) -> u64 {
    let group_split: Vec<&str> = input.split("\n\n").collect();
    let rules = group_split[0];

    let mut my_rules = HashMap::new();
    println!("Hi");
    
    for line in rules.lines() {
        let first_split: Vec<&str> = line.split("{").collect();
        let name = first_split[0];
        
        let mut my_rule = Rule {
            name,
            maps: vec![]
        };
        let second_split: Vec<&str> = first_split[1].split("}").collect();
        let comma_split_list: Vec<(u64, &str)>  = second_split[0].split(",").enumerate().map(|(id, thing)| (id as u64, thing)).collect();
        for (id, mapping) in &comma_split_list{
            
            let new_map;
            if *id == (comma_split_list.len() - 1) as u64 {
                new_map = Mapping {
                    catch_all: true,
                    is_lt: false,
                    var: "x",
                    num: 10,
                    new_name: &mapping,
                };
            } else {
            
                let mut saved_ch = ' ';
                for ch in mapping.chars() {
                    if ch == '>' {
                        saved_ch = '>';
                        break;
                    }
                    if ch == '<' {
                        saved_ch = '<';
                        break;
                    }
                }
                let var_split: Vec<&str> = mapping.split(saved_ch).collect();
                let var = var_split[0];
                let tag_split: Vec<&str> = var_split[1].split(":").collect();
                let tag = tag_split[1];
                let val = tag_split[0].parse().unwrap();
                new_map = Mapping {
                    catch_all: false,
                    is_lt: saved_ch == '<',
                    var, 
                    num: val,
                    new_name: tag,
                };
            }
            my_rule.maps.push(new_map);
        }
        my_rules.insert(my_rule.name, my_rule.maps);
    }

    let mut point_list = vec![];
    
    point_list.push(Point {
        tag: "in",
        x: (1, 4001),
        m: (1, 4001),
        a: (1, 4001),
        s: (1, 4001),
    });

    let mut accepted = vec![];
    
    let mut first = true;
    let mut i = 0;
    while point_list.len() != 0 {
        if first {
            println!("------------------------------------");
        }
        if i % 1000000 == 0 {
            println!("{}", point_list.len());
        }
        if i == 10000000 {
            break;
        }
        let mut current_point = point_list.pop().unwrap();
        if first {
            println!("Current_point: {:#?}", current_point);
        }
        if first {
            println!("Current_stack: {:#?}", point_list);
        }
        if let None = my_rules.get(current_point.tag) {
            println!("tag: {}", current_point.tag);
        }
        let rule = &my_rules[current_point.tag];
        for mapping in rule {
            
            if first {
                println!("Current_mapping: {:#?}", mapping);
            }
            if mapping.catch_all {
                current_point.tag = mapping.new_name;
                println!("ctachal tag {}", current_point.tag);
                if current_point.tag == "A" {
                    if first {
                        println!("catchall is pushed to accepted");
                    }
                    println!("Success_range is pushed to accepted");
                    accepted.push(current_point);
                } else if current_point.tag == "R" {
                    if first {
                        println!("Catchall is pushed to rejected");
                    }
                } else {
                    if first {
                        println!("Catchall is pushed to on stack");
                    }
                    point_list.push(current_point);
                }
                break;
            }

            let val = get_point_val_from_name(mapping.var, &current_point); 

            if first {
                println!("Val: {:#?}", val);
            }
            let (success_range, fail_range) = compare(mapping.is_lt, val, mapping.num);

            if first {
                println!("Success_range: {:#?}", success_range);
            }
            if first {
                println!("fail_rule: {:#?}", fail_range);
            }

            let mut success_point = current_point.clone();
            let mut failure_point = current_point.clone();

            set_val_from_name(mapping.var, &mut success_point, success_range);
            set_val_from_name(mapping.var, &mut failure_point, fail_range);
            success_point.tag = mapping.new_name; 

            if first {
                println!("Success point: {:#?}", success_point);
            }
            if first {
                println!("Failure point: {:#?}", failure_point);
            }

            let mut skip = false;
            if success_range.1 - success_range.0 != 0 {
                if first {
                    println!("Success_range != (0,0)");
                }
                if success_point.tag == "A" {
                    println!("Success_range is pushed to accepted");
                    accepted.push(success_point);
                } else if success_point.tag == "R" {
                    if first {
                        println!("Success_range is pushed to rejected");
                    }
                } else {
                    if first {
                        println!("Success_range is pushed to on stack");
                    }
                    point_list.push(success_point);
                }
                skip = true;
            }
            if skip {
                if fail_range.1 - fail_range.0 != 0 {
                    if first {
                        println!("Fail range is pushed to stack");
                    }
                    point_list.push(failure_point);
                }
                if first {
                    println!("Skipping next mapping because sucessful mapping found");
                }
                break;
            }
        }
        i += 1;
        if i == 5 {
            first = false;
        }
    }

    let mut sum = 0;
    for point in accepted {
        let lenx = point.x.1 - point.x.0;
        let lenm = point.m.1 - point.m.0;
        let lena = point.a.1 - point.a.0;
        let lens = point.s.1 - point.s.0;
        sum += lenx * lenm * lena * lens; 
    }

    return sum;
}

fn range_sum(range: (u64, u64)) -> u64 {
    ((range.1 - 1 )* (range.1)) / 2 - ((range.0 - 1) * (range.0)) / 2
}

fn compare(is_lt: bool, range: (u64, u64), right: u64) -> ((u64, u64), (u64, u64)) {
    if is_lt {
        if range.1 < right {
            return (range, (0,0));
        } else if range.0 >= right {
            return ((0,0), range);
        } else {
            return range_split(range, right);
        }
    } else {
        if range.0 > right {
            return (range, (0,0));
        } else if range.1 <= right {
            return ((0,0), range);
        } else {
            let result = range_split(range, right + 1);
            return (result.1, result.0);
        }
    }
}

fn range_split(range: (u64, u64), location: u64) -> ((u64, u64), (u64, u64)) {
    return ((range.0, location), (location, range.1));
}

fn set_val_from_name(input: &str, point: &mut Point, range: (u64, u64)) {
    match input {
        "x" => point.x = range,
        "m" => point.m = range,
        "a" => point.a = range,
        "s" => point.s = range,
        val => panic!("variable not defined on point for set {}", val),
    }
}

fn get_point_val_from_name(input: &str, point: &Point) -> (u64,u64) {
    match input {
        "x" => point.x,
        "m" => point.m,
        "a" => point.a,
        "s" => point.s,
        val => panic!("Variable not defined on point {}", val),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example() {
        let result = parse(r"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
");
        assert_eq!(result, 167409079868000);
    }

    #[test]
    fn range_sum_test() {
        let result = range_sum((1, 4001));

        assert_eq!(result, 8_002_000);
    }

    #[test]
    fn range_split_test() {
        let result = range_split((1, 4001), 2000);

        assert_eq!(result, ((1, 2000), (2000, 4001)));
    }

    #[test]
    fn comparison_split_test_lt() {
        let result = compare(true, (1, 4001), 2000);

        assert_eq!(result, ((1, 2000), (2000, 4001)));
    }

    #[test]
    fn comparison_split_test_gt() {
        let result = compare(false, (1, 4001), 2000);

        assert_eq!(result, ((2001, 4001), (1, 2001)));
    }
    
}
