fn main() {
    let input = include_str!("./input.txt");
    let output = parse(input);
    dbg!(output);
}

fn parse(input: &str) -> usize {
    let mut sum = 0;

    for input_section in input.split("\n\n") {
        let mut grid = vec![];
        for line in input_section.lines() {
            let mut row = vec![];
            for char in line.chars() {
                row.push(char);
            }
            grid.push(row);
        }
       
        for row in 1..grid.len() - 1 {
            if check_horizontal_mirror(&grid, row) {
                sum += (row + 1) * 100;
            }
        }

        for col in 1..grid[0].len() - 1 {
            if check_vertical_mirror(&grid, col) {
                sum += col + 1;
            }
        }

    }

    return sum;
}

fn print_row(
    grid: &Vec<Vec<char>>,
    row: usize
) {
    for i in 0..grid[row].len() {
        print!("{}", grid[row][i]);
    }
    print!("\n");
}

fn check_vertical_mirror(
    grid: &Vec<Vec<char>>, 
    col: usize
) -> bool {
   
    // col = left side of divide
    // loop through left side of mirror
    for i in 0..=col {

        // calculate right side mirroring
        let dist = col - i;
        let mirror_col = col + 1 + dist;

        if mirror_col >= grid[0].len() {
            continue;
        }
        for row in 0..grid.len() {
            if grid[row][i] != grid[row][mirror_col] {
                return false;
            }
        }
        
    }
    return true;
}

fn check_horizontal_mirror(
    grid: &Vec<Vec<char>>, 
    row: usize
) -> bool {

    for i in 0..=row {

        let dist = row - i;
        let mirror_row = row + 1 + dist;

    if row == 6 {
        println!("row, mirror_row {}, {}", i, mirror_row);
    }

        if mirror_row >= grid.len() {
            continue;
        }

    if row == 6 {
        print_row(grid, i);
        print_row(grid, mirror_row);
    }
        for col in 0..grid[0].len() {
            if grid[i][col] != grid[mirror_row][col] {
                return false;
            }
        }
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example() {
        let result = parse(r"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
");
        assert_eq!(result, 405);
    }

    #[test]
    fn parse_input_txt() {
        let result = parse(r"##...######...##.
......#..#.......
##.##########.###
###..........###.
...#..#..#..#...#
##..###..###..###
..##.#.##.#.##..#
..####.##.####..#
####........####.
..#.##.##.##.#..#
##.###.##.###.###
###.#......#.####
......####......#
#..#.##..##.#..##
..#.########.#...
##...######...##.
..#.##....##.#..#
");
        assert_eq!(result, 8);
    }

    #[test]
    fn parse_non_centered() {
        let result = parse(r"....##.
#..#..#
.##....
####..#
#..####
....##.
#......
");
        assert_eq!(result, 5);
    }

    #[test]
    fn parse_vert_non_centered() {
        let result = parse(r".........##.#.#
..#######....##
.###...####..#.
....#.#.#.##...
.###.#.#....#.#
#.##...##...##.
.##...#.#.....#
.##...#.#.....#
#.##...##...##.
.###.#.#....#.#
....#.#.#.##...
.###...####....
..#######....##
.........##.#.#
.........##.#.#
");
        assert_eq!(result, 1400);
    }

    #[test]
    fn parse_more() {
        let result = parse(r"############.##
#..######..#.##
#...####...####
...######......
#.###..###.#.##
#..##..##..####
.###....###.###
#.#......#.#.##
..########..##.
");
        assert_eq!(result, 6);
    }
}
