use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


#[allow(dead_code)]
fn part1() {
    let mut races: Vec<(i32, i32)> = Vec::new();
    if let Ok(lines) = read_lines("./inputs/input_day6.txt") {
        for line in lines {
            if let Ok(line) = line {
                if line.contains("Time") {
                    let times = line.split(":").nth(1).unwrap().trim().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
                    for time in times {
                        races.push((time, 0));
                    }
                } else {
                    let distances = line.split(":").nth(1).unwrap().trim().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
                    for (i, distance) in distances.iter().enumerate() {
                        races[i].1 = *distance;
                    }
                }
            }
        }
    }

    let mut product = 1;
    for (t, d) in races {
        let mut c = 0;
        for hold_for in 1..t {
            let new_d = (t - hold_for)*hold_for;
            if new_d > d {
                c += 1; 
            }
        }
        product = product * c;

    }
    println!("{}", product);


}

#[allow(unused)]
fn main() {

    let mut time:i64 = 0;
    let mut distance:i64 = 0;
    if let Ok(lines) = read_lines("./inputs/input_day6.txt") {
        for line in lines {
            if let Ok(line) = line {
                if line.contains("Time") {
                    time = line.split(":").nth(1).unwrap().trim().split_whitespace().collect::<Vec<&str>>().join("").parse::<i64>().unwrap();
                } else {
                    distance = line.split(":").nth(1).unwrap().trim().split_whitespace().collect::<Vec<&str>>().join("").parse::<i64>().unwrap();
                }
            }
        }
    }

    
    let mut c = 0;
    for hold_for in 1..time {
        let new_d = (time - hold_for)*hold_for;
        if new_d > distance {
            c += 1; 
        }
    }
    println!("{}", c);
        

    

}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}