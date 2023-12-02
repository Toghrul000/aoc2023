use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[allow(dead_code)]
fn part1()->i64{
    let mut sum = 0;
    let mut stack: Vec<char> = Vec::new();
    
    if let Ok(lines) = read_lines("./inputs/input_day1.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                for c in line.chars() {
                    if c.is_digit(10){
                        stack.push(c);

                    }
                }
                let mut number = String::new();
                if stack.len() > 1 {
                    number.push(*stack.first().unwrap());
                    number.push(*stack.last().unwrap());
                    sum += number.parse::<i64>().unwrap();
                    
                } else if stack.len() == 1 {
                    if let Some(n) = stack.pop() {
                        number.push(n);
                        number.push(n);
                        sum += number.parse::<i64>().unwrap();
                        

                    }
      
                }
                stack.clear();
                


            }
        }
        println!("Sum is {}", sum);
    }
    return sum;


}



#[allow(unused)]
fn main() {
    let mut sum = 0;
    let mut stack: Vec<char> = Vec::new();
    let numbers = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    
    if let Ok(lines) = read_lines("./inputs/input_day1.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                for (i, c) in line.char_indices() {
                    if c.is_digit(10){
                        stack.push(c);

                    }
                    for (d, val) in numbers.iter().enumerate() {
                        if line[i..].starts_with(val){
                            stack.push(char::from_digit((d+1) as u32, 10).unwrap());
                        }
                    }



                }
                let mut number = String::new();
                number.push(*stack.first().unwrap());
                number.push(*stack.last().unwrap());
                sum += number.parse::<i64>().unwrap();
                stack.clear();
                


            }
        }
        println!("Sum is {}", sum);
    }


}