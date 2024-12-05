use topological_sort::TopologicalSort;
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

    let get_index = |values: &Vec<i32>, x: i32| values.iter().position(|n| *n==x);

    let mut invalid_lists = vec![];
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

        if !valid {
            invalid_lists.push(items);
        }
    }

    for list in invalid_lists {
        let mut output = vec![];

        let mut ts = TopologicalSort::<i32>::new();

        for (a,b) in &asserts_groups {
            if list.contains(a) && list.contains(b) {
                ts.add_dependency(*a, *b);
            }
        }
        let mut order = vec![];

        while let Some(val) = ts.pop() {
            order.push(val);
        }

        for item in &order {
            if list.contains(item) {
                output.push(*item);
            }
        } 
        for item in &list {
            if !output.contains(&item) {
                output.push(*item);
            }
        }

        assert!(output.len() == list.len());
        sum += output[output.len() / 2];
    }

    return sum;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example() {
        let result = parse(r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
");
        assert_eq!(result, 123);
    }

}
