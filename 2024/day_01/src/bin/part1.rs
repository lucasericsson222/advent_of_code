fn main() {
    let input = include_str!("./input1.txt");
    let output = parse(input);
    dbg!(output);
}

fn parse(input: &str) -> i32 {
    let mut sum = 0;

    let mut list1: Vec<i32> = vec![];
    let mut list2: Vec<i32> = vec![];

    for line in input.lines() {
        let mut innerelement = vec![]; 
        for item in line.split_whitespace() {
            innerelement.push(item.parse::<i32>().unwrap());
        }
        list1.push(innerelement[0]);
        list2.push(innerelement[1]);
    }

    list1.sort();
    list2.sort();


    for (a, b) in list1.into_iter().zip(list2.into_iter()) {
        sum += (a - b).abs(); 
    }

    return sum;
}
