use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


#[allow(unused)]
pub fn main() {
    let mut init_sequence: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("./inputs/input_day15.txt") {
        for line in lines {
            if let Ok(line) = line {
                init_sequence = line.trim().split(",").map(|s| s.to_string()).collect();

            } 
        }
    }
    
    let mut sum = 0;
    for s in init_sequence {
        let mut current_value = 0;
        for c in s.chars(){
            current_value += c as u32;
            current_value = current_value * 17;
            current_value = current_value % 256;
    
        }
        sum+=current_value;
    }

    println!("Part 1 {}", sum);


}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}