fn main() {
    let input = include_str!("./input1.txt");
    let output = parse(input);
    dbg!(output);
}

fn parse(input: &str) -> i32 {
    let mut sum = 0;

    let mut grid: Vec<Vec<char>>= vec![];

    for line in input.lines() {
        let mut row = vec![];
        for ch in line.chars() {
            row.push(ch);
        }
        grid.push(row);
    }

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            sum += check_neighbors(i, j, 'X', &grid, None);
        }
    }

    return sum;
}

fn check_neighbors(i: usize, j: usize, current_letter: char, grid: &Vec<Vec<char>>, dir: Option<(i32, i32)>) -> i32 {
    let next_letter = match current_letter {
            'X' => 'M',
            'M' => 'A',
            'A' => 'S',
            'S' => 'F',
            'F' => '.',
            _ => todo!(),
        };
    
    if grid[i][j] != current_letter {
        return 0; 
    }
    if next_letter == 'F' {
        return 1;
    }

    let mut res = 0;
    if let Some(dir) = dir {
        let new_i = (i as i32 + dir.0);
        let new_j = (j as i32 + dir.1);
        if !(new_i < 0 || new_i >= grid.len() as i32) && 
            !(new_j < 0 || new_j >= grid[i].len() as i32) {
            res += check_neighbors(new_i as usize, new_j as usize, next_letter, grid, Some(dir));
        }
    } else {
        for o_i in -1..=1 {
            for o_j in -1..=1 {
                if o_i == 0 && o_j == 0 {
                    continue;
                }
                let new_i = (i as i32 + o_i);
                let new_j = (j as i32 + o_j);
                if new_i < 0 || new_i >= grid.len() as i32 {
                    continue;
                }
                if new_j < 0 || new_j >= grid.len() as i32 {
                    continue;
                }
                res += check_neighbors(new_i as usize, new_j as usize, next_letter, grid, Some((o_i, o_j)));
            }
        }
    }
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn parse_example1() {
        let result = parse(r"....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX
");
        assert_eq!(result, 18);
    }
    #[test]
    fn parse_example() {
        let result = parse(r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
");
        assert_eq!(result, 18);
    }

}
