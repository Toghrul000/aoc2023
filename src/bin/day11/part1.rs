use std::cmp::min;
use std::cmp::max;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


#[allow(unused)]
pub fn main() {

    let mut grid: Vec<String> = Vec::new();

    if let Ok(lines) = read_lines("./inputs/input_day11.txt") {
        for line in lines {
            if let Ok(line) = line {
                grid.push(line);
            }
        }
    }
    let mut row_dots_indexes: Vec<usize> = grid
    .iter()
    .enumerate()
    .filter(|(_, row)| row.chars().all(|ch| ch == '.'))
    .map(|(index, _)| index)
    .collect();

    let mut col_dots_indexes: Vec<usize> = (0..grid[0].len())
    .filter(|&c| grid.iter().all(|row| row.chars().nth(c) == Some('.')))
    .collect();

    let mut galaxy_coordinates: Vec<(usize, usize)> = grid
    .iter()
    .enumerate()
    .flat_map(|(r, row)| {
        row.chars()
            .enumerate()
            .filter(|(_, ch)| *ch == '#')
            .map(move |(c, _)| (r, c))
    })
    .collect();

    let mut sum: i32 = 0;
    let expansion = 2;

    for (i, (r1, c1)) in galaxy_coordinates.iter().enumerate() {
        for (r2, c2) in galaxy_coordinates[..i].to_vec() {
            for r in min(*r1, r2)..max(*r1, r2){
                sum += if row_dots_indexes.contains(&r) { expansion} else {1}
            }
            for c in min(*c1, c2)..max(*c1, c2){
                sum += if col_dots_indexes.contains(&c) { expansion} else {1}
            }

        }
    }

    println!("Part 1: {sum}");

    



    



}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}