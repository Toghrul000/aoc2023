use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
use std::collections::VecDeque;
use regex::Regex;


#[allow(unused)]
pub fn main() {

    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut deq: VecDeque<(usize, usize)> = VecDeque::new();
    if let Ok(lines) = read_lines("./inputs/input_day10.txt") {
        for (r, line) in lines.into_iter().enumerate() {
            grid.insert(r, Vec::new());
            if let Ok(line) = line {
                for (c, ch) in line.trim().chars().enumerate() {
                    grid[r].push(ch);
                    if ch == 'S' {
                        seen.insert((r, c));
                        deq.push_back((r, c));
                    }
                }
                
            } 
        }
    }
    //println!("{:?}", grid);

    let mut maybe_s = HashSet::from(['|', '-', 'J', 'L', '7', 'F']);
    while !deq.is_empty() {
        let (r, c) = deq.pop_front().unwrap();
        let ch = grid[r][c];

        // Top
        if r > 0 && "S|JL".contains(ch) && "|7F".contains(grid[r - 1][c]) && !seen.contains(&(r-1, c)) {
            seen.insert((r - 1, c));
            deq.push_back((r - 1, c));
            if ch == 'S' {
                maybe_s.retain(|&x| HashSet::from(['|', 'J', 'L']).contains(&x));
            }
        }

        // Down
        if grid.len() - 1 > r && "S|7F".contains(ch) && "|JL".contains(grid[r + 1][c]) && !seen.contains(&(r+1, c)) {
            seen.insert((r + 1, c));
            deq.push_back((r + 1, c));
            if ch == 'S' {
                maybe_s.retain(|&x| HashSet::from(['|', '7', 'F']).contains(&x));
            }
        }

        // Left
        if c > 0 && "S-J7".contains(ch) && "-LF".contains(grid[r][c - 1]) && !seen.contains(&(r, c - 1)) {
            seen.insert((r, c - 1));
            deq.push_back((r, c - 1));
            if ch == 'S' {
                maybe_s.retain(|&x| HashSet::from(['-', 'J', '7']).contains(&x));
            }
        }

        // Right
        if grid[r].len() - 1 > c && "S-LF".contains(ch) && "-J7".contains(grid[r][c + 1]) && !seen.contains(&(r, c+1)) {
            seen.insert((r, c + 1));
            deq.push_back((r, c + 1));
            if ch == 'S' {
                maybe_s.retain(|&x| HashSet::from(['-', 'L', 'F']).contains(&x));
            }
        }

    }

    grid.iter_mut().for_each(|row| {
        if let Some(index) = row.iter().position(|&x| x == 'S') {
            std::mem::replace(&mut row[index], *maybe_s.iter().next().unwrap());
        }
    });

    for (r, row) in grid.iter_mut().enumerate() {
        for (c, ch) in row.iter_mut().enumerate() {
            if !seen.contains(&(r, c)) {
                *ch = '.';
                
            }
        }
    }


    let mut new_grid:Vec<String> = Vec::new();
    grid.iter().for_each(|row| {
        new_grid.push(row.into_iter().collect());
    });
    let mut inside = 0;
    let re = Regex::new(r"L-*J|F-*7").unwrap();

    for (r, row) in new_grid.iter().enumerate() {
        let cleaned_row = re.replace_all(row, "");
        let mut within = false;
        for ch in cleaned_row.chars(){
            if "|FL".contains(ch) {
                within = !within;
            }
            if within && ch == '.' {
                inside += 1;
            }
        }
    }

    println!("Part 2 is {}", inside);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}