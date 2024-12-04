fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let grid: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();

    let mut sum = 0;
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == 'A' {
                if find_xmas(&grid, row, col) == 1 {
                    println!("{} {}", row, col);
                    sum += 1;
                }
            }
        }
    }

    println!("{}", sum);
}

fn find_xmas(grid: &Vec<Vec<char>>, row: usize, col: usize) -> i32 {
    let last_row = grid.len() - 1;
    let last_col = grid[0].len() - 1;

    let mut total = 0;

    if (row >= 1 && col >= 1) && (row + 1 <= last_row && col + 1 <= last_col) {
        // upleft downright
        // upright downleft
        if ((grid[row - 1][col - 1] == 'M' && grid[row + 1][col + 1] == 'S')
            || (grid[row + 1][col + 1] == 'M' && grid[row - 1][col - 1] == 'S'))
            && ((grid[row - 1][col + 1] == 'M' && grid[row + 1][col - 1] == 'S')
                || (grid[row + 1][col - 1] == 'M' && grid[row - 1][col + 1] == 'S'))
        {
            total += 1;
        }
    }

    return total;
}
