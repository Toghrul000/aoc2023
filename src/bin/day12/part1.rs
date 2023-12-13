use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


#[allow(unused)]
pub fn main() {
    let mut records: Vec<(String, Vec<usize>)> = Vec::new();
    if let Ok(lines) = read_lines("./inputs/input_day12.txt") {
        for line in lines {
            if let Ok(line) = line {
                let mut parts = line.trim().split_whitespace();
                let mut spring_conditons = parts.next().unwrap().to_string();
                //println!("{}", spring_conditons);
                let group_sizes: Vec<usize> = parts.next().unwrap().split(",").map(|x| x.parse::<usize>().unwrap()).collect();
                records.push((spring_conditons, group_sizes));
            } 
        }
    }

    let mut sum = 0;
    for (string, sizes) in records {
        sum += count_recursive(&string, &sizes, 0);
        

    }

    println!("Part 1 is {}", sum);

}

fn count_recursive(cfg: &str, nums: &[usize], index: usize) -> usize {
    //println!("{}", index);
    if index >= cfg.len() {
        return if nums.is_empty() { 1 } else { 0 };
    }

    if nums.is_empty() {
        return if cfg[index..].contains('#') { 0 } else { 1 };
    }

    let mut result = 0;

    if cfg[index..].chars().next().unwrap() == '.' || cfg[index..].chars().next().unwrap() == '?' {
        result += count_recursive(cfg, nums, index + 1);
    }

    if cfg[index..].chars().next().unwrap() == '#' || cfg[index..].chars().next().unwrap() == '?' {
        if nums[0] <= cfg.len() - index && !cfg[index..index + nums[0]].contains('.') &&
            (nums[0] == cfg.len() - index || cfg.chars().nth(index + nums[0]).unwrap() != '#')
        {
            result += count_recursive(cfg, &nums[1..], index + nums[0] + 1);
        }
    }

    result
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}