fn main() {
    let input = include_str!("./input1.txt");
    let output = parse(input);
    dbg!(output);
}

fn parse(input: &str) -> usize {
    let mut state = vec![]; 
    
    for num in input.split(",") {
        let num = num.parse::<usize>().unwrap();
        state.push(num);
    }

    for i in 0..80 {
        println!("{}", i);
        let mut new_fish = vec![];
        for mut fish in &mut state {
            if *fish == 0 {
                *fish = 6;
                new_fish.push(8);
            } else {
                *fish -= 1;
            }
        }
        state.extend(new_fish.iter());
    }

    return state.len();
}
