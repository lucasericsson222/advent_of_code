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
            if check_for_xmas(i, j, &grid) {
                sum += 1;
            }
        }
    }

    return sum;
}

fn check_for_xmas(i: usize, j: usize, grid: &Vec<Vec<char>>) -> bool {
    if i < 1 || i >= grid.len() - 1 {
        return false;
    }
    if j < 1 || j >= grid.len() - 1{
        return false;
    }
    if grid[i][j] != 'A' {
        return false;
    }

    let mut count = 0;
    if grid[i-1][j-1] == 'S' && grid[i+1][j+1] == 'M' {
        count += 1;
    } else if grid[i-1][j-1] == 'M' && grid[i+1][j+1] == 'S' {
        count += 1;
    } 
    if grid[i-1][j+1] == 'S' && grid[i+1][j-1] == 'M' {
        count += 1;
    } else if grid[i-1][j+1] == 'M' && grid[i+1][j-1] == 'S' {
        count += 1;
    } 
    if count == 2{
        return true;
    }
    return false;

}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn parse_example1() {
        let result = parse(r".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........
");
        assert_eq!(result, 9);
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
        assert_eq!(result, 9);
    }

}
