fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let grid: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();

    let mut sum = 0;
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == 'X' {
                sum += find_xmas(&grid, row, col);
            }
        }
    }

    println!("{}", sum);
}

fn find_xmas(grid: &Vec<Vec<char>>, row: usize, col: usize) -> i32 {
    let last_row = grid.len() - 1;
    let last_col = grid[0].len() - 1;

    let mut total = 0;
    // search to the right
    if col + 3 <= last_col {
        if grid[row][col + 1] == 'M' && grid[row][col + 2] == 'A' && grid[row][col + 3] == 'S' {
            total += 1;
        }
    }

    // search to the left
    if col >= 3 {
        if grid[row][col - 1] == 'M' && grid[row][col - 2] == 'A' && grid[row][col - 3] == 'S' {
            total += 1;
        }
    }

    // search down
    if row + 3 <= last_row {
        if grid[row + 1][col] == 'M' && grid[row + 2][col] == 'A' && grid[row + 3][col] == 'S' {
            total += 1;
        }
    }

    // search up
    if row >= 3 {
        if grid[row - 1][col] == 'M' && grid[row - 2][col] == 'A' && grid[row - 3][col] == 'S' {
            total += 1;
        }
    }

    // search up right
    if row >= 3 && col + 3 <= last_col {
        if grid[row - 1][col + 1] == 'M'
            && grid[row - 2][col + 2] == 'A'
            && grid[row - 3][col + 3] == 'S'
        {
            total += 1;
        }
    }

    // search up left
    if row >= 3 && col >= 3 {
        if grid[row - 1][col - 1] == 'M'
            && grid[row - 2][col - 2] == 'A'
            && grid[row - 3][col - 3] == 'S'
        {
            total += 1;
        }
    }

    // search down left
    if row + 3 <= last_row && col >= 3 {
        if grid[row + 1][col - 1] == 'M'
            && grid[row + 2][col - 2] == 'A'
            && grid[row + 3][col - 3] == 'S'
        {
            total += 1;
        }
    }

    // search down right
    if row + 3 <= last_row && col + 3 <= last_col {
        if grid[row + 1][col + 1] == 'M'
            && grid[row + 2][col + 2] == 'A'
            && grid[row + 3][col + 3] == 'S'
        {
            total += 1;
        }
    }

    return total;
}
