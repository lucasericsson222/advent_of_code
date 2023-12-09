fn main() {
    let input = include_str!("./input.txt");
    let output = parse(input);
    dbg!(output);
}

fn parse(input: &str) -> i32 {
    let lines = input.lines();
    let mut sum = 0;
    for line in lines {

        let mut rows = vec![];

        rows.push(
            line
                .split_whitespace()
                .map(|value| value.parse::<i32>().unwrap())
                .collect()
        );
        let mut current = 0;
        while !is_zero_vec(&rows[current]) {
            let mut new_vec = vec![];
            let previous_row = &rows[current];
            for i in 0..(rows[current].len()-1) {
                new_vec.push(previous_row[i + 1] - previous_row[i]); 
            }
            rows.push(new_vec);
            current += 1;
        }
        let mut temp_sum = 0;
        for row in rows {
            temp_sum += row.last().unwrap();
        }
        sum += temp_sum;
    }

    return sum;
}

fn is_zero_vec(myvec: &Vec<i32>) -> bool {
    for value in myvec {
        if *value != 0 {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
}
