fn main() {
    let input =include_str!("./input.txt");
    let output = parse(input);
    dbg!(output);
}

fn parse(input: &str) -> usize {
    let mut count = 0;
    for (_, line) in input.lines().enumerate() {
        let hash_record = line
            .split_whitespace()
            .nth(0)
            .unwrap()
            .chars()
            .collect();
        let num_record: Vec<usize> = line
            .split_whitespace()
            .nth(1)
            .unwrap()
            .split(',')
            .map(|value| value.parse::<usize>().unwrap())
            .collect();


        println!("{:#?}", hash_record);
        println!("{:#?}", num_record); 
        let temp_count = count_placement_ways(0, 0, &hash_record, &num_record);

        if temp_count == 0 {
            panic!();
        }
        count += temp_count; 
    }

    return count;
}

fn count_placement_ways(hash_index: usize, num_index: usize, hash_record: &Vec<char>, num_record: &Vec<usize>) -> usize {
    
    println!("i, j {}, {}", hash_index, num_index);
    if num_index >= num_record.len() && no_future_hash(hash_index, hash_record) {
        return 1;
    } else if num_index >= num_record.len() {
        return 0;
    }
    if hash_index >= hash_record.len() {
        return 0;
    }

    if hash_record[hash_index] == '#' {
        println!("hash");
        let max_size = count_hash_question_marks(hash_index, hash_record); 
        println!("max_size = {}", max_size);
        if num_record[num_index] <= max_size {
            println!("eating hash");
            if hash_index + num_record[num_index] < hash_record.len() {
                if hash_record[hash_index + num_record[num_index]] == '#' {
                    return 0;
                }
            }
            return count_placement_ways(hash_index + 1 + num_record[num_index], num_index + 1, hash_record, num_record); 
        }
        return 0;
    } else if hash_record[hash_index] == '.' {
        println!("dot");
        return count_placement_ways(hash_index + 1, num_index, hash_record, num_record);
    } else {
        println!("Questionmark");
        let max_size = count_hash_question_marks(hash_index, hash_record);
        if num_record[num_index] <= max_size {
            if hash_index + num_record[num_index] < hash_record.len() {
                if hash_record[hash_index + num_record[num_index]] == '#' {
                    println!("next_index");
                    return count_placement_ways(hash_index + 1, num_index, hash_record, num_record)
                }
            }
            println!("eating questionmarks and hash / next_index");
            return 
                count_placement_ways(hash_index + 1 + num_record[num_index], num_index + 1, hash_record, num_record)
                + count_placement_ways(hash_index + 1, num_index, hash_record, num_record);
        } 
        return count_placement_ways(hash_index + 1, num_index, hash_record, num_record);
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
    fn parse_first_line_example() {
        let result = parse(r"#.#.### 1,1,3");

        assert_eq!(result, 1);
    }

    #[test]
    fn parse_second_line_example() {
        let result = parse(r".??..??...?##. 1,1,3");

        assert_eq!(result, 4);
    }

    #[test]
    fn parse_third_line_example() {
        let result = parse(r"?#?#?#?#?#?#?#? 1,3,1,6");

        assert_eq!(result, 1);
    }

    #[test]
    fn parse_fourth_line_example() {
        let result = parse(r"????.#...#... 4,1,1");

        assert_eq!(result, 1);
    }

    #[test]
    fn parse_fifth_line_example() {
        let result = parse(r"????.######..#####. 1,6,5");

        assert_eq!(result, 4);
    }

    #[test]
    fn parse_sixth_line_example() {
        let result = parse(r"?###???????? 3,2,1");

        assert_eq!(result, 10);
    }

    #[test]
    fn parse_example() {
        let result = parse(r"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
");
        assert_eq!(result, 21);
    }

    #[test]
    fn parse_input_line_1() {
        let result = parse(r"?????????#?# 1,1,7");
        //???? 1,1,7
        assert_eq!(result, 3); 
    }

    #[test]
    fn parse_input_line_2() {
        let result = parse(r"??.?????????#??? 3,4");
        //?????????#??? 3,4
        //###   ???#??? 4
        // ###  ???#??? 4
        //  ### ???#??? 4
        //   ### ??#??? 3
        //    ### ?#??? 2
        //     ### #### 1
        assert_eq!(result, 18);
    }

    #[test]
    fn parse_input_line_3() {
        let result = parse(r"?###?????.???.# 3,2,1,2,1");

        assert_eq!(result,2);
    }

    #[test]
    fn parse_input_line_4() {
        let result = parse(r"?.??...??# 2,3");
        //  ??...??# 2,3
        assert_eq!(result, 1);
    }

    #[test]
    fn parse_input_line_5() {
        let result = parse(r"?...??#??##..?.#?? 1,7,1,1");
        //?............?.#?? 1,1,1
        //#............?.#?? 2 

        assert_eq!(result, 2);
    }

    #[test]
    fn parse_reddit_case() {
        let result = parse(r"#.#?. 1,1");

        assert_eq!(result, 1);
    }

    #[test]
    fn parse_random_case() {
        let result = parse(r"?.???.#.??##????#?? 1,2,1,4,3,1");
        //#.???.#.??##????#?? 1,2,1,4,3,1
        // * 2
        //??##????#?? 4,3,1
        //#### ?### #
        // #### ### #
        //
        assert_eq!(result, 4);
    }

    #[test]
    fn parse_random_case_1() {
        let result = parse(r"..?..#???.#??? 1,1,1,4"); 

        assert_eq!(result, 2);

    }

    #[test]
    fn parse_all_hash() {
        let result = parse(r"#.#..##.##.# 1,1,2,2,1"); 

        assert_eq!(result, 1);
    }

    #[test]
    fn my_test_1() {
        let result = parse(r"???.#??#.???? 3,1,1");

        assert_eq!(result, 1);
    }

    #[test]
    fn parse_reddit_case_2() {
        let result = parse(r".##.?#??.#.?# 2,1,1,1");

        assert_eq!(result, 1);
    }

    #[test]
    fn parse_reddit_case_3() {
        let result = parse(r"???#??.??????.??#.. 4,3");
        assert_eq!(result, 3);
    }

    #[test]
    fn parse_reddit_case_4() {
        let result = parse(r"?#?###???#??#?.??? 11,1,1
.????#?#???#??????? 12,1,1
?#???##??#?????#.??? 2,11,1
#??##???????????. 1,11,1
.??.?.?#?##?#???#?? 1,11
??#.?#???####??#??.? 1,11,1");
        assert_eq!(result, 31);
    }

    #[test]
    fn parse_reddit_case_5() {
        let result = parse(r"???# 2,1");

        assert_eq!(result, 1);
    }

}
