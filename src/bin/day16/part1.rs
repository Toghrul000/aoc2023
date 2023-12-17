use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::usize;

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Coordinates {
    x: usize,
    y: usize,
 }
 #[derive(Debug, Eq, Hash, PartialEq, Clone)]
 enum Direction {
    Right,
    Left,
    Up,
    Down
 }

#[allow(unused)]
pub fn main() {

    let mut grid: Vec<Vec<char>> = Vec::new();
    if let Ok(lines) = read_lines("./inputs/input_day16.txt") {
        for line in lines {
            if let Ok(line) = line {
                grid.push(line.chars().collect());
            } 
        }
    }
    let mut starts: Vec<(Coordinates, Direction)> = Vec::new();
    let mut seen: HashSet<(Coordinates, Direction)> = HashSet::new();
    starts.push((Coordinates { x: 0, y: 0 }, Direction::Right));

    loop {
        //println!("yolo");
        if starts.len() == 0 {
            break;
        }
        let (mut light, mut direction) = starts.pop().unwrap();
        loop {
            let mut b_flag = false;
            if seen.contains(&(Coordinates {x: light.x, y: light.y}, direction.clone())){
                break;
            }
            seen.insert((Coordinates {x: light.x, y: light.y}, direction.clone()));
    
            match direction {
                Direction::Right => {
                    if grid[light.x][light.y] == '/' {
                        if let Some(i) = light.x.checked_sub(1){
                            light.x = i;
                            direction = Direction::Up;
                        } else {
                            b_flag = true;
                        }
                    } else if grid[light.x][light.y] == '\\' {
                        if light.x + 1 < grid.len() {
                            light.x = light.x + 1;
                            direction = Direction::Down;
                        } else {
                            b_flag = true;
                        }
                    } else if grid[light.x][light.y] == '|' {
                        let old_x = light.x;
                        let old_y = light.y;
                        // continue
                        if light.x + 1 < grid.len() {
                            light.x = light.x + 1;
                            direction = Direction::Down;
                        } else {
                            b_flag = true;
                        }

                        // continue later
                        if let Some(i) = old_x.checked_sub(1){
                            // println!("{:?}", grid[light.x][light.y] == '|');
                            // println!("light x {}, y {}", light.x, light.y);
                            // println!("light x {}, {}", light.x, i);
                            starts.push((Coordinates { x: i, y: old_y }, Direction::Up))
                        } 
                    } else {
                        if light.y + 1 < grid[light.x].len() {
                            light.y += 1;
                        } else {
                            b_flag = true
                        }
                    }
                },
                Direction::Left => {
                    if grid[light.x][light.y] == '/' {
                        if light.x + 1 < grid.len() {
                            light.x = light.x + 1;
                            direction = Direction::Down;
                        } else {
                            b_flag = true;
                        }
                    } else if grid[light.x][light.y] == '\\' {
                        if let Some(i) = light.x.checked_sub(1){
                            light.x = i;
                            direction = Direction::Up;
                        } else {
                            b_flag = true;
                        }

                    } else if grid[light.x][light.y] == '|' {
                        let old_x = light.x;
                        let old_y = light.y;
                        if light.x + 1 < grid.len() {
                            light.x = light.x + 1;
                            direction = Direction::Down;
                        } else {
                            b_flag = true;
                        }

                        if let Some(i) = old_x.checked_sub(1){
                            starts.push((Coordinates { x: i, y: old_y }, Direction::Up))
                        } 
                    } else {
                        if let Some(i) = light.y.checked_sub(1) {
                            light.y = i;
                        } else {
                            b_flag = true
                        }
                    }
                },
                Direction::Up => {
                    if grid[light.x][light.y] == '/' {
                        if light.y + 1 < grid[light.x].len() {
                            light.y += 1;
                            direction = Direction::Right;
                        } else {
                            b_flag = true;
                        }
                    } else if grid[light.x][light.y] == '\\' {
                        if let Some(i) = light.y.checked_sub(1) {
                            light.y = i;
                            direction = Direction::Left;
                        } else {
                            b_flag = true
                        }
                    } else if grid[light.x][light.y] == '-' {
                        let old_x = light.x;
                        let old_y = light.y;
                        if light.y + 1 < grid[light.x].len() {
                            light.y += 1;
                            direction = Direction::Right;
                        } else {
                            b_flag = true;
                        }

                        if let Some(i) = old_y.checked_sub(1) {
                            starts.push((Coordinates { x: old_x, y: i }, Direction::Left))
                        }
                    } else {
                        if let Some(i) = light.x.checked_sub(1){
                            light.x = i
                        } else {
                            b_flag = true;
                        }
                    }

                },
                Direction::Down => {
                    if grid[light.x][light.y] == '/' {
                        if let Some(i) = light.y.checked_sub(1) {
                            light.y = i;
                            direction = Direction::Left;
                        } else {
                            b_flag = true
                        }
                    } else if grid[light.x][light.y] == '\\' {
                        if light.y + 1 < grid[light.x].len() {
                            light.y += 1;
                            direction = Direction::Right;
                        } else {
                            b_flag = true;
                        }
                    } else if grid[light.x][light.y] == '-' {
                        let old_x = light.x;
                        let old_y = light.y;
                        if light.y + 1 < grid[light.x].len() {
                            light.y += 1;
                            direction = Direction::Right;
                        } else {
                            b_flag = true;
                        }

                        if let Some(i) = old_y.checked_sub(1) {
                            starts.push((Coordinates { x: old_x, y: i }, Direction::Left))
                        }
                    } else {
                        if light.x + 1 < grid.len() {
                            light.x = light.x + 1;
                        } else {
                            b_flag = true;
                        }
                    }
                },
            }
            if b_flag {
                break;
            }
        }
        //break;
    }
    //println!("{:?}", seen);

    let coordinates_only_set: HashSet<Coordinates> = seen
    .iter()
    .map(|(coordinates, _)| coordinates.clone())
    .collect();

    //println!("{:?}", coordinates_only_set);
    let mut sum = 0;
        
    for _ in &coordinates_only_set {
        sum+=1;
    }
    println!("Part 1 is {}", sum);

    

}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}