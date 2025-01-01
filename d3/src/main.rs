use std::fs;

fn check(grid:&Vec<Vec<char>>,dx:usize,dy:usize) -> i64 {
    let mut x = 0;
    let mut cnt = 0;
    for i in (0..grid.len()).step_by(dy) {
        if grid[i][x]=='#' {
            cnt = cnt + 1;
        }
        x = (x + dx) % grid[0].len();
    }
    cnt
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    let grid:Vec<Vec<char>> = contents.lines().map(|s| s.chars().collect()).collect();

    let total = check(&grid,1,1) * check(&grid,3,1) * check(&grid,5,1) * check(&grid,7,1) * check(&grid,1,2);
    println!("{}",total);
}
