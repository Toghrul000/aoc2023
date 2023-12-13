use std::cmp::min;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn find_reflection(pattern: Vec<String>) -> usize{
    for r in 1..pattern.len() {
        let mut left = pattern[..r].to_vec();
        left.reverse();

        let mut right = pattern[r..].to_vec();

        
        left = left[..min(right.len(), left.len())].to_vec();
        right = right[..min(right.len(), left.len())].to_vec();

        let mut sum: usize = 0;
        for (x, y) in left.iter().zip(&right) {
            for (a, b) in x.chars().into_iter().zip(y.chars().into_iter()){
                if a != b {
                    sum+=1;
                } 
            }


        }

        if sum == 1 {
            return r;
        }


    }
    return 0
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

#[allow(unused)]
pub fn main() {

    let mut patterns: Vec<Vec<String>> = Vec::new();
    if let Ok(lines) = read_lines("./inputs/input_day13.txt") {
        let mut index = 0;
        patterns.push(Vec::new());
        for line in lines {
            if let Ok(line) = line {
                let clean_line = line.trim().to_string();
                if clean_line != "" {
                    patterns[index].push(line);
                } else {
                    patterns.push(Vec::new());
                    index+=1;

                }
            } 
        }
    }

    let mut sum = 0;
    patterns.iter().for_each(|v| {

        sum+=find_reflection(v.clone())*100;

        let mut v1 = v.iter()
        .map(|s| s.chars().collect())
        .collect();

        let transposed_v: Vec<String> = transpose(v1).iter()
        .map(|char_vec| char_vec.iter().collect())
        .collect();;

        sum+=find_reflection(transposed_v);


    });

    println!("Part 2 {}", sum);

}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}