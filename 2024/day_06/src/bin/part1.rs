fn main() {
    let input = include_str!("./input1.txt");
    let output = parse(input);
    dbg!(output);
}

fn parse(input: &str) -> i32 {
    let mut sum;


    let mut grid = vec![];
    for line in input.lines() {
        let mut row = vec![];
        for char in line.chars() {
            row.push(char);
        }
        grid.push(row);
    }

    let mut pos = find_character_pos(&grid);
    let mut dir = (-1, 0);

    while move_character(&mut grid, &mut pos, &mut dir) {}

    sum = count_num_x(&grid);

    return sum;
}

fn count_num_x(grid: &Vec<Vec<char>>) -> i32 {
    let mut sum = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'X' {
                sum += 1;
            }
        }
    }
    sum
}

fn move_character(grid: &mut Vec<Vec<char>>, pos: &mut (i32, i32), dir: &mut (i32, i32)) -> bool {
    print!("{},{}\n", pos.0, pos.1);
    if pos.0 < 0 || pos.0 >= grid.len() as i32 {
        return false;
    }
    if pos.1 < 0 || pos.1 >= grid[0].len() as i32 {
        return false;
    }

    grid[pos.0 as usize][pos.1 as usize] = 'X';
    let mut newpos = (pos.0 + dir.0, pos.1 + dir.1);

    if newpos.0 < 0 || newpos.0 >= grid.len() as i32 {
        return false;
    }
    if newpos.1 < 0 || newpos.1 >= grid[0].len() as i32 {
        return false;
    }
    if grid[newpos.0 as usize][newpos.1 as usize] == '#' {
        *dir = match dir {
            (-1, 0) => (0, 1),
            (0, 1) => (1, 0),
            (1, 0) => (0, -1),
            (0, -1) => (-1, 0),
            _ => panic!("WHAT YOU DOING"),
        };
        newpos = (pos.0 + dir.0, pos.1 + dir.1);
    } 

    *pos = newpos;

    return true;
}

fn find_character_pos(grid: &Vec<Vec<char>>) -> (i32, i32) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '^' {
                return (i as i32, j as i32);
            }
        }
    }
    panic!();
}


#[cfg(test)]
mod tests {
    use super::*;

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
