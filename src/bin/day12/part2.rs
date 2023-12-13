use std::collections::HashMap;
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

                let initial = spring_conditons.clone();
                for s in 0..4 {
                    spring_conditons.push('?');
                    spring_conditons.push_str(initial.as_str());
                }

                let mut group_sizes: Vec<usize> = parts.next().unwrap().split(",").map(|x| x.parse::<usize>().unwrap()).collect();

                let mut init_sizes = group_sizes.clone();
                for s in 0..4 {
                    group_sizes.extend(init_sizes.clone());

                }
                //println!("{} {:?}", spring_conditons, group_sizes);
                records.push((spring_conditons, group_sizes));
            } 
        }
    }
    let mut cache: HashMap<(String, Vec<usize>, usize), usize> = HashMap::new();

    let mut sum = 0;
    for (string, sizes) in records {

        sum += count_recursive(string, sizes, 0, &mut cache);
    }

    println!("Part 2 is {}", sum);

}

fn count_recursive(cfg: String, nums: Vec<usize>, index: usize, cache: &mut HashMap<(String, Vec<usize>, usize), usize>) -> usize {
    //println!("{}", index);
    if index >= cfg.len() {
        return if nums.is_empty() { 1 } else { 0 };
    }

    if nums.is_empty() {
        return if cfg[index..].contains('#') { 0 } else { 1 };
    }

    if cache.contains_key(&(cfg.clone(), nums.clone(), index)) {
        return *cache.get(&(cfg.clone(), nums, index)).unwrap();
    }

    let mut result = 0;

    if cfg[index..].chars().next().unwrap() == '.' || cfg[index..].chars().next().unwrap() == '?' {
        result += count_recursive(cfg.clone(), nums.clone(), index + 1, cache);
    }

    if cfg[index..].chars().next().unwrap() == '#' || cfg[index..].chars().next().unwrap() == '?' {
        if nums[0] <= cfg.len() - index && !cfg[index..index + nums[0]].contains('.') &&
            (nums[0] == cfg.len() - index || cfg.chars().nth(index + nums[0]).unwrap() != '#')
        {
            result += count_recursive(cfg.clone(), nums[1..].to_vec(), index + nums[0] + 1, cache);
        }
    }

    cache.insert((cfg, nums, index), result);

    result
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}