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
struct Mapping<'a> {
    catch_all: bool,
    is_lt: bool,
    var: &'a str,
    num: usize,
    new_name: &'a str,
}

struct Point<'a> {
    tag: &'a str,
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

fn parse(input: &str) -> usize {
    let group_split: Vec<&str> = input.split("\n\n").collect();
    let rules = group_split[0];
    let points = group_split[1];

    let mut my_rules = HashMap::new();
    
    for line in rules.lines() {
        let first_split: Vec<&str> = line.split("{").collect();
        let name = first_split[0];
        
        let mut my_rule = Rule {
            name,
            maps: vec![]
        };
        let second_split: Vec<&str> = first_split[1].split("}").collect();
        let comma_split_list: Vec<(usize, &str)>  = second_split[0].split(",").enumerate().collect();
        for (id, mapping) in &comma_split_list{
            
            let new_map;
            if *id == comma_split_list.len() - 1 {
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
    for line in points.lines() {
        let new_line = &line[1..line.len()-1];
        let new_line = new_line
            .replace("x", "")
            .replace("m", "")
            .replace("a", "")
            .replace("s", "")
            .replace("=", "");
        let out: Vec<usize> = new_line.split(",").map(|val| val.parse().unwrap()).collect();
        
        let point = Point {
            tag: "in",
            x: out[0],
            m: out[1],
            a: out[2],
            s: out[3],
        };
        point_list.push(point);
    }

    let mut accepted = vec![];

    while point_list.len() != 0 {
        let mut current_point = point_list.pop().unwrap();

        let rule = &my_rules[current_point.tag];
        
        for mapping in rule {
            if mapping.catch_all {
                current_point.tag = mapping.new_name;
                break;
            }
            let val = get_point_val_from_name(mapping.var, &current_point); 
            if compare(mapping.is_lt, val, mapping.num) {
                current_point.tag = mapping.new_name;
                break;
            }
        }
        if current_point.tag == "A" {
            accepted.push(current_point);
        } else if current_point.tag == "R" {
            continue;
        } else {
            point_list.push(current_point);
        }
    }
    
    accepted.iter().map(|val| val.x + val.m + val.a + val.s).sum()
}

fn compare(is_lt: bool, left: usize, right: usize) -> bool {
    if !is_lt {
        left > right
    } else {
        left < right
    }
}

fn get_point_val_from_name(input: &str, point: &Point) -> usize {
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
        assert_eq!(result, 19114);
    }
    
}
