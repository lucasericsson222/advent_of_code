use std::{mem::swap, ops::Range};

fn main() {
    let input = include_str!("./input1.txt");
    let output = parse(input);
    dbg!(output);
}

#[derive(Clone)]
struct IdRange {
    range: usize,
    id: Option<usize>,
}


fn parse(input: &str) -> i64 {

    let mut vals = vec![];

    for (i, ch) in input.chars().enumerate() {
        let val = ch.to_digit(10).unwrap();
        if i % 2 == 0 {
            vals.push(IdRange {range: val as usize, id: Some(i/2)});
        } else {
            vals.push(IdRange {range: val as usize, id: None});
        }
    }

    let mut n = vals.iter().filter_map(|x| x.id).max().unwrap();
    while let Some(settle) = settle_blocks(&mut vals, n) {
        if n == 0 {
            break;
        }
        n -= 1;
        print!("{}\n", n);
        //conglomorate_empty(&mut vals);
    }


    let mut sum: i64 = 0;
    let mut cumulative_index = 0;
    for val in vals {
        if let Some(id) = val.id {
            for j in 0..val.range {
                //print!("{}", id);
                sum += id as i64 * (cumulative_index) as i64;
                cumulative_index += 1;
            }
        } else {
            for _ in 0..val.range {
                //print!(".");
                cumulative_index += 1;
            }
        }
    }
    return sum;
}

fn settle_blocks(vals: &mut Vec<IdRange>, n: usize) -> Option<bool> {
    let mut block = None;
    let mut loc = None;

    // get nth last block
    for i in (0..vals.len()).rev() {
        let block_to_move = &vals[i];        
        if block_to_move.id.is_none() {
            continue;
        }
        if block_to_move.id.unwrap() == n {
            block = Some(block_to_move);
            loc = Some(i);
            break; 
        }
    }
    let block = block?;

    for j in 0..loc? {
        let block_to_replace = &vals[j];
        if block_to_replace.id != None {
            continue;
        }
        if block_to_replace.range < block.range {
            continue;
        }
        let new_block = IdRange {range: block_to_replace.range - block.range, id: None};
        if new_block.range == 0 {
            vals[j].id = vals[loc?].id;
            vals[loc?].id = None;
        } else {
            vals[j] = IdRange {range: block.range, id: block.id};
            vals[loc?].id = None;
            vals.insert(j+1, new_block);
        }
        return Some(true);
    }
    return Some(false);
}

fn conglomorate_empty(vals: &mut Vec<IdRange>) {
    let mut newvals = vec![];
    
    while vals.len() > 0 {
        let mut cur = vec![];

        for val in vals.iter() {
            if cur.len() == 0 {
                cur.push(val.clone());
                continue;
            }
            if cur[0].id == None && val.id == None {
                cur.push(val.clone()); 
            } else {
                break;
            }
        }

        vals.drain(0..cur.len());

        let mut size = 0;
        if cur[0].id == None {
            for el in cur {
                size += el.range;
            }
            newvals.push(IdRange {range: size, id: None});
        } else {
            newvals.push(cur[0].clone());
        }
    }

    *vals = newvals;
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example() {
        let result = parse(r"");
        assert_eq!(result, 18);
    }

}
