use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


#[allow(unused)]
pub fn main() {

    //let mut dig_plan: Vec<(String, usize, String)> = Vec::new();
    let mut points: Vec<(i32, i32)> = Vec::new();
    points.push((0, 0));
    let mut dirmap: HashMap<String, (i32, i32)> = HashMap::new();
    dirmap.insert("U".to_string(), (-1, 0));
    dirmap.insert("D".to_string(), (1, 0));
    dirmap.insert("L".to_string(), (0, -1));
    dirmap.insert("R".to_string(), (0, 1));
    let mut b = 0;
    if let Ok(lines) = read_lines("./inputs/input_day18.txt") {
        for line in lines {
            if let Ok(line) = line {
                let mut parts = line.trim().split_whitespace();
                let dir = parts.next().unwrap().to_string();
                let (dr, dc) = dirmap[&dir];
                let steps = parts.next().unwrap().parse::<i32>().unwrap();
                b+=steps;
                let color = parts.next().unwrap().to_string();
                let (r, c) = points.last().unwrap();
                points.push((r + dr * steps, c + dc*steps));
                
            } 
        }
    }
    let len = points.len();

    let area: i32 = points.iter().enumerate().map(|(i, &(x, y))| {
        x * (points[(i+len-1)%len].1 - points[(i+1)%len].1)
    }).sum::<i32>().abs() / 2;

    let i = area - (b/2) + 1;

    println!("Part 1 {}", i + b);

}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}