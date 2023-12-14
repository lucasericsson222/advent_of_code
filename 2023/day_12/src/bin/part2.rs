use std::collections::HashMap;

fn main() {
    let input =include_str!("./input.txt");
    let output = parse(input);
    dbg!(output);
}

fn parse(input: &str) -> usize {
    let mut count = 0;
    for (_, line) in input.lines().enumerate() {
        let hash_record_intermediate: Vec<char> = line
            .split_whitespace()
            .nth(0)
            .unwrap()
            .chars()
            .collect();
        let num_record_intermediate: Vec<usize> = line
            .split_whitespace()
            .nth(1)
            .unwrap()
            .split(',')
            .map(|value| value.parse::<usize>().unwrap())
            .collect();
        

        let mut hash_record = vec![];
        let mut num_record = vec![];

        for i in 0..5 {
            for val in &hash_record_intermediate {
                hash_record.push(*val);
            }
            for val in &num_record_intermediate {
                num_record.push(*val);
            }

            if i != 4 {
                hash_record.push('?');
            }

            
        }
        let mut memo_map = HashMap::new();
        
        let temp_count = memoize(&mut memo_map, 0, 0, &hash_record, &num_record);

        if temp_count == 0 {
            panic!();
        }
        count += temp_count; 
    }

    return count;
}

fn memoize(
    memo_map: &mut HashMap<(usize, usize), usize>,
    hash_index: usize, 
    num_index: usize, 
    hash_record: &Vec<char>, 
    num_record: &Vec<usize>
) -> usize {
    if memo_map.contains_key(&(hash_index, num_index)) {
        return memo_map[&(hash_index, num_index)];
    }

    let val = count_placement_ways(memo_map, hash_index, num_index, hash_record, num_record);
    memo_map.insert((hash_index, num_index), val);
    return val;
} 

fn count_placement_ways(
    memo_map: &mut HashMap<(usize, usize), usize>,
    hash_index: usize, 
    num_index: usize, 
    hash_record: &Vec<char>, 
    num_record: &Vec<usize>
) -> usize {
    
    if num_index >= num_record.len() && no_future_hash(hash_index, hash_record) {
        return 1;
    } else if num_index >= num_record.len() {
        return 0;
    }
    if hash_index >= hash_record.len() {
        return 0;
    }

    if hash_record[hash_index] == '#' {
        let max_size = count_hash_question_marks(hash_index, hash_record); 
        if num_record[num_index] <= max_size {
            if hash_index + num_record[num_index] < hash_record.len() {
                if hash_record[hash_index + num_record[num_index]] == '#' {
                    return 0;
                }
            }
            return memoize(memo_map, hash_index + 1 + num_record[num_index], num_index + 1, hash_record, num_record); 
        }
        return 0;
    } else if hash_record[hash_index] == '.' {
        return memoize(memo_map, hash_index + 1, num_index, hash_record, num_record);
    } else {
        let max_size = count_hash_question_marks(hash_index, hash_record);
        if num_record[num_index] <= max_size {
            if hash_index + num_record[num_index] < hash_record.len() {
                if hash_record[hash_index + num_record[num_index]] == '#' {
                    return memoize(memo_map, hash_index + 1, num_index, hash_record, num_record)
                }
            }
            return 
                memoize(memo_map, hash_index + 1 + num_record[num_index], num_index + 1, hash_record, num_record)
                + memoize(memo_map, hash_index + 1, num_index, hash_record, num_record);
        } 
        return memoize(memo_map, hash_index + 1, num_index, hash_record, num_record);
    }
}

fn no_future_hash(start_index: usize, hash_record: &Vec<char>) -> bool {
    for i in start_index..hash_record.len() {
        if hash_record[i] == '#' {
            return false;
        }
    }
    return true;
}

fn count_hash_question_marks(start_index: usize, hash_record: &Vec<char>) -> usize {
    let mut count = 0;

    for i in start_index..hash_record.len() {
        if hash_record[i] == '.' {
            break;
        }
        count += 1;
    }

    return count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example() {
        let result = parse(r"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
");
        assert_eq!(result, 525152);
    }
}
