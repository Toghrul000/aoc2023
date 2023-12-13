use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn check_column_helper(pattern: Vec<String>, index_of_reflection: usize) -> bool {
    let col_len = pattern.len();
    let row_len = pattern[0].len();

    let mut columns: Vec<String> = Vec::new();

    for c in 0..row_len {
        let mut column = String::new();
        for r in 0..col_len {
            column.push(pattern[r].chars().nth(c).unwrap());
        }
        columns.push(column);
    }
    let mut left = columns[0..index_of_reflection].to_vec();
    let right = columns[index_of_reflection..columns.len()].to_vec();

    if right.len() < left.len() {
        left.reverse();
        for (i, vr) in right.iter().enumerate() {
            if *vr != left[i] {
                return false;
            }
        }
    } else {
        left.reverse();

        for (i, vl) in left.iter().enumerate() {
            if *vl != right[i] {
                return false;
            }
        }
    }

    return true;
}


fn check_column(pattern: Vec<String>) -> Vec<usize> {
    let col_len = pattern.len();
    let row_len = pattern[0].len();

    let mut prev_column = String::new();
    let mut possible_indexes: Vec<usize> = vec![0];
    for c in 0..row_len {
        let mut column = String::new();
        for r in 0..col_len {
            column.push(pattern[r].chars().nth(c).unwrap());
        }
        if column == prev_column && c != 0 {
            if check_column_helper(pattern.clone(), c) {
                possible_indexes.push(c);
            } 
        }

        prev_column = column.clone();
    }
    return possible_indexes;
}

fn check_row_helper(pattern: Vec<String>, index_of_reflection: usize) -> bool {
    let mut left = pattern[0..index_of_reflection].to_vec();
    let right = pattern[index_of_reflection..pattern.len()].to_vec();

    // left.iter().for_each(|x| {
    //     println!("{}", x);
    // });
    // println!();
    // right.iter().for_each(|x| {
    //     println!("{}", x);
    // });
    

    if right.len() < left.len() {
        left.reverse();
        for (i, vr) in right.iter().enumerate() {
            if *vr != left[i] {
                return false;
            }
        }
    } else {
        left.reverse();

        for (i, vl) in left.iter().enumerate() {
            if *vl != right[i] {
                return false;
            }
        }
    }

    return true;
}


fn check_row(pattern: Vec<String>) -> Vec<usize> {
    let col_len = pattern.len();
    let row_len = pattern[0].len();

    let mut prev_row = String::new();
    let mut possible_indexes: Vec<usize> = vec![0];
    for c in 0..col_len {
        let mut row = String::new();
        for r in 0..row_len {
            row.push(pattern[c].chars().nth(r).unwrap());
        }

        if row == prev_row && c != 0{
            //println!("possible {}", c);
            if check_row_helper(pattern.clone(), c) {
                possible_indexes.push(c);
            }
        }

        prev_row = row.clone();
    }
    return possible_indexes;
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
        let c = *check_column(v.clone()).iter().max().unwrap();
        let r = *check_row(v.clone()).iter().max().unwrap();

        //println!("{:?}, {:?}", c, r);

        if c > r {
            sum += c;
        } else {
            sum += r*100;  
        }

    });

    println!("Part 1 {}", sum);




    

}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}