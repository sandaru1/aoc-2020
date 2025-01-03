use std::fs;
use std::fmt;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone)]
enum GridPos {
    Floor,
    Empty,
    Occupied
}

impl fmt::Debug for GridPos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GridPos::Floor => write!(f, "."),
            GridPos::Empty => write!(f, "L"),
            GridPos::Occupied => write!(f, "#"),
        }
    }
}

fn is_all_empty(grid:&Vec<Vec<GridPos>>,row:usize,col:usize)->bool {
    let directions = [
        (0, 1), (0, -1), (1, 0), (-1, 0),
        (1, 1), (1, -1), (-1, 1), (-1, -1),
    ];

    directions.iter().all(|&(dr,dc)| {
        let r = row as i8 + dr;
        let c = col as i8 + dc;
        if r < 0 || c < 0 || r >= grid.len() as i8 || c >= grid[0].len() as i8 {
            return true;
        }
        grid[r as usize][c as usize] == GridPos::Empty || grid[r as usize][c as usize] == GridPos::Floor
    })
}

fn count_occupied(grid:&Vec<Vec<GridPos>>,r:usize,c:usize)->usize {
    let directions = [
        (0, 1), (0, -1), (1, 0), (-1, 0),
        (1, 1), (1, -1), (-1, 1), (-1, -1),
    ];

    directions.iter().filter(|&&(dr,dc)| {
        let r = r as i8 + dr;
        let c = c as i8 + dc;
        if r < 0 || c < 0 || r >= grid.len() as i8 || c >= grid[0].len() as i8 {
            return false;
        }
        grid[r as usize][c as usize] == GridPos::Occupied
    }).count()
}

fn step(grid:&Vec<Vec<GridPos>>)->Option<Vec<Vec<GridPos>>> {
    let mut new_grid = grid.clone();
    let mut changed = false;
    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            match grid[r][c] {
                GridPos::Empty if is_all_empty(grid, r, c) => {
                    new_grid[r][c] = GridPos::Occupied;
                    changed = true;
                }
                GridPos::Occupied if count_occupied(grid, r, c) >= 4 => {
                    new_grid[r][c] = GridPos::Empty;
                    changed = true;
                }
                _ => {}
            }
        }
    }

    if changed { Some(new_grid) } else { None }
}

fn get_visible_seats(grid:&Vec<Vec<GridPos>>,row:usize,col:usize)->Vec<(usize,usize)> {
    let directions = [
        (0, 1), (0, -1), (1, 0), (-1, 0),
        (1, 1), (1, -1), (-1, 1), (-1, -1),
    ];

    directions.iter().map(|&(dr,dc)| {
        let mut r = row as i8 + dr;
        let mut c = col as i8 + dc;
        while r >= 0 && c >= 0 && r < grid.len() as i8 && c < grid[0].len() as i8 {
            if grid[r as usize][c as usize] != GridPos::Floor {
                return Some((r as usize,c as usize));
            }
            r += dr;
            c += dc;
        }
        None
    }).collect::<Vec<_>>().iter().filter_map(|&x| x).collect()    
}

fn is_all_empty2(grid:&Vec<Vec<GridPos>>,row:usize,col:usize,visible_seats:&HashMap<(usize,usize),Vec<(usize,usize)>>)->bool {
    if let Some(visible_seats) = visible_seats.get(&(row,col)) {
        visible_seats.iter().all(|&(r,c)| {
            grid[r][c] == GridPos::Empty || grid[r][c] == GridPos::Floor
        })
    } else {
        false
    }
}

fn count_occupied2(grid:&Vec<Vec<GridPos>>,row:usize,col:usize,visible_seats:&HashMap<(usize,usize),Vec<(usize,usize)>>)->usize {
    if let Some(visible_seats) = visible_seats.get(&(row,col)) {
        visible_seats.iter().filter(|&&(r,c)| {
            grid[r][c] == GridPos::Occupied
        }).count()
    } else {
        0
    }
}

fn step2(grid:&Vec<Vec<GridPos>>,visible_seats:&HashMap<(usize,usize),Vec<(usize,usize)>>)->Option<Vec<Vec<GridPos>>> {
    let mut new_grid = grid.clone();
    let mut changed = false;
    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            match grid[r][c] {
                GridPos::Empty if is_all_empty2(grid, r, c, visible_seats) => {
                    new_grid[r][c] = GridPos::Occupied;
                    changed = true;
                }
                GridPos::Occupied if count_occupied2(grid, r, c, visible_seats) >= 5 => {
                    new_grid[r][c] = GridPos::Empty;
                    changed = true;
                }
                _ => {}
            }
        }
    }

    if changed { Some(new_grid) } else { None }
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    let mut grid:Vec<Vec<GridPos>> = contents.lines().map( |line| {
            line.chars().map(|c| match c {
                '#' => GridPos::Occupied,
                'L' => GridPos::Empty,
                '.' => GridPos::Floor,
                _ => panic!("Invalid character in input"),
            }).collect()
        }).collect();

    let mut visible_seats:HashMap<(usize,usize),Vec<(usize,usize)>> = HashMap::new();
    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            visible_seats.insert((r,c),get_visible_seats(&grid,r,c));
        }
    }

    while let Some(next_grid) = step2(&grid,&visible_seats) {
        grid = next_grid;
    }

    let occupied_count:usize = grid
        .iter()
        .map(|row| row.iter().filter(|&c| *c==GridPos::Occupied).count())
        .sum();
    println!("{}",occupied_count);
}
