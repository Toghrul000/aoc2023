use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


#[allow(unused)]
pub fn main() {
    let mut sum: i64 = 0;
    if let Ok(lines) = read_lines("./inputs/input_day9.txt") {
        for line in lines {
            if let Ok(line) = line {
                let mut diff_vector: Vec<Vec<i64>> = Vec::new();
                let mut history: Vec<i64> = line.trim().split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect();
                diff_vector.push(history.clone());
                loop {
                    let last_row = diff_vector.last().unwrap().clone();
                    let mut diff: Vec<i64> = last_row.windows(2).map(|w| w[1] - w[0]).collect();
                    if diff.iter().all(|&x| x == 0) {
                        diff_vector.push(diff);
                        break;
                    }
                    diff_vector.push(diff);
                }

                diff_vector.reverse();
                // diff_vector.iter_mut().map(|x: &mut Vec<i64>| x.reverse());



                for i in 0..diff_vector.len() {
                    if diff_vector[i].iter().all(|&x| x == 0) {
                        diff_vector[i].push(0);
                    } else {
                        
                        let last = diff_vector[i].first().unwrap().clone();
                        let last_prev = diff_vector[i - 1].first().unwrap().clone();

                        diff_vector[i].insert(0, last - last_prev);
                    }
                }
                sum += diff_vector.last().unwrap().first().unwrap();


            } 
        }
    } 

    println!("Part 2 answer is {}", sum);

}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}