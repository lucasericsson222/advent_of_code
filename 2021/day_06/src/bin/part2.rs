use std::collections::HashMap;

fn main() {
    let input = include_str!("./input2.txt");
    let output = parse(input);
    dbg!(output);
}

fn parse(input: &str) -> u64 {
    let mut items = HashMap::new(); 
    for num in input.split(",") {
        let num = num.parse::<u64>().unwrap();
        items.entry(num).and_modify(|x| *x += 1).or_insert(0);
    }
    for i in 0..80 {
        println!("{}", i);
        let mut new_hashmap = HashMap::new();
        for (num, count) in items {
            if i == 0 {
                println!("{},{}", num, count);
            }
            if num == 0 {
                new_hashmap.entry(8).and_modify(|x| *x += count).or_insert(count);
                new_hashmap.entry(6).and_modify(|x| *x += count).or_insert(count);
            } else {
                new_hashmap.entry(num-1).and_modify(|x| *x += count).or_insert(count);
            }
        }
        items = new_hashmap;
    }
    let mut sum = 0;
    for (num, count) in items {
        sum += count;
    }
    return sum;
}
