use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


#[allow(unused)]
fn main() {

    if let Ok(lines) = read_lines("./inputs/input_day4.txt") {
        for line in lines {
            if let Ok(line) = line {
 
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