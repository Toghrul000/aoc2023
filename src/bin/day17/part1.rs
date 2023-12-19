use std::collections::{HashSet, BinaryHeap};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp::Reverse;

#[allow(unused)]
pub fn main() {
    let mut grid: Vec<Vec<i32>> = Vec::new();
    if let Ok(lines) = read_lines("./inputs/input_day17.txt") {
        for line in lines {
            if let Ok(line) = line {
                grid.push(line.chars().map(|c| c.to_string().parse::<i32>().unwrap()).collect());
            }
        }
    }

    let mut seen: HashSet<(i32, i32, i32, i32, i32)> = HashSet::new();
    let mut pq: BinaryHeap<Reverse<(i32, i32, i32, i32, i32, i32)>> = BinaryHeap::new();
    pq.push(Reverse((0, 0, 0, 0, 0, 0)));

    while let Some(Reverse((hl, r, c, dr, dc, n))) = pq.pop() {
        //println!("{} {} {} {} {} {}",hl, r, c, dr, dc, n);
        if r == (grid.len() - 1) as i32 && c == (grid[0].len() - 1) as i32 {
            println!("Part 1 is {}", hl);
            break;
        }

        if seen.contains(&(r, c, dr, dc, n)){
            continue;
        }
        seen.insert((r, c, dr, dc, n));

        if n < 3 && (dr, dc) != (0, 0){
            let nr = r + dr;
            let nc = c + dc;
            if 0 <= nr && nr < grid.len() as i32 && 0 <= nc && nc < grid[0].len() as i32 {
                pq.push(Reverse((hl + grid[nr as usize][nc as usize], nr, nc, dr, dc, n + 1)));
            }
        }

        for (ndr, ndc) in vec![(0, 1), (1, 0), (0, -1), (-1, 0)] {
            if (ndr, ndc) != (dr, dc) && (ndr, ndc) != (-dr, -dc){
                let nr = r + ndr;
                let nc = c + ndc;
                if 0 <= nr && nr < grid.len() as i32 && 0 <= nc && nc < grid[0].len() as i32 {
                    pq.push(Reverse((hl + grid[nr as usize][nc as usize], nr, nc, ndr, ndc, 1)));
                }
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
