use std::collections::HashSet;

fn main() {
    let input = include_str!("./input1.txt");
    let output = parse(input);
    dbg!(output);
}

fn parse(input: &str) -> i32 {
    let mut sum = 0;


    let mut grid = vec![];
    for line in input.lines() {
        let mut row = vec![];
        for char in line.chars() {
            row.push(char);
        }
        grid.push(row);
    }


    for i in 0..grid.len() {
        print!("{}\n", i);
        for j in 0..grid[i].len() {
            if grid[i][j] == '#' {
                continue;
            }
            if grid[i][j] == '^' {
                continue;
            }
            let mut pos = find_character_pos(&grid);
            let mut dir = (-1, 0);
            grid[i][j] = '#';
            
            let mut prevplaces = HashSet::<((i32, i32), (i32, i32))>::new();
            let mut iscycle = false;
            while move_character(&mut grid, &mut pos, &mut dir, &mut prevplaces, &mut iscycle) {}

            if iscycle {
                sum += 1;
            } 

            grid[i][j] = '.';
        }
    } 

    return sum;
}

fn move_character(
    grid: &mut Vec<Vec<char>>, 
    pos: &mut (i32, i32), 
    dir: &mut (i32, i32), 
    previous_places: &mut HashSet<((i32, i32), (i32, i32))>,
    iscycle: &mut bool) -> bool {
        
    if pos.0 < 0 || pos.0 >= grid.len() as i32 {
        return false;
    }
    if pos.1 < 0 || pos.1 >= grid[0].len() as i32 {
        return false;
    }
    
    if previous_places.contains(&(*pos, *dir)) {
        *iscycle = true;
        return false;
    }

    previous_places.insert((pos.clone(), dir.clone()));

    //grid[pos.0 as usize][pos.1 as usize] = 'X';

    let mut newpos = (pos.0 + dir.0, pos.1 + dir.1);

    if newpos.0 < 0 || newpos.0 >= grid.len() as i32 {
        return false;
    }
    if newpos.1 < 0 || newpos.1 >= grid[0].len() as i32 {
        return false;
    }
    while grid[newpos.0 as usize][newpos.1 as usize] == '#' {
        *dir = match dir {
            (-1, 0) => (0, 1),
            (0, 1) => (1, 0),
            (1, 0) => (0, -1),
            (0, -1) => (-1, 0),
            _ => panic!("WHAT YOU DOING"),
        };
        newpos = (pos.0 + dir.0, pos.1 + dir.1);
        if newpos.0 < 0 || newpos.0 >= grid.len() as i32 {
            return false;
        }
        if newpos.1 < 0 || newpos.1 >= grid[0].len() as i32 {
            return false;
        }
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
