use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
use std::collections::VecDeque;


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

    while !deq.is_empty() {
        let (r, c) = deq.pop_front().unwrap();
        let ch = grid[r][c];

        // Top
        if r > 0 && "S|JL".contains(ch) && "|7F".contains(grid[r - 1][c]) && !seen.contains(&(r-1, c)) {
            seen.insert((r - 1, c));
            deq.push_back((r - 1, c));
        }

        // Down
        if grid.len() - 1 > r && "S|7F".contains(ch) && "|JL".contains(grid[r + 1][c]) && !seen.contains(&(r+1, c)) {
            seen.insert((r + 1, c));
            deq.push_back((r + 1, c));
        }

        // Left
        if c > 0 && "S-J7".contains(ch) && "-LF".contains(grid[r][c - 1]) && !seen.contains(&(r, c - 1)) {
            seen.insert((r, c - 1));
            deq.push_back((r, c - 1));
        }

        // Right
        if grid[r].len() - 1 > c && "S-LF".contains(ch) && "-J7".contains(grid[r][c + 1]) && !seen.contains(&(r, c+1)) {
            seen.insert((r, c + 1));
            deq.push_back((r, c + 1));
        }
    }
    println!("Part 1 answer: {:?}", seen.len()/2);

}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}