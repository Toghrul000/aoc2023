use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;


#[allow(unused)]
pub fn main() {

    let mut navigation: Vec<char> = Vec::new();
    let mut nodes: HashMap<String, (String, String)> = HashMap::new();
    if let Ok(lines) = read_lines("./inputs/input_day8.txt") {
        for line in lines {
            if let Ok(line) = line {
                if !line.trim().is_empty(){

                    if !line.contains("=") {
                        for (i, c) in line.trim().to_string().chars().enumerate() {
                            navigation.push(c);
                        }
                    } else {
                        let mut parts = line.trim().split("=");
                        let key = parts.next().unwrap().trim().to_string();
                        let mut ways = parts.next().unwrap().trim().trim_start_matches("(").trim_end_matches(")").split(",");
                        let left = ways.next().unwrap().trim().to_string();
                        let right = ways.next().unwrap().trim().to_string();
                        nodes.insert(key, (left, right));

                    }
                }
            } 
        }
    }
    navigation.reverse();

    //println!("{:?}", nodes);

    let mut start = "AAA".to_string();
    let mut temp_navigation = navigation.clone();
    let mut steps = 0;

    while let Some(c) = temp_navigation.pop() {
        if c == 'R' {
            start = nodes.get(&start).unwrap().1.clone();

        } else {
            start = nodes.get(&start).unwrap().0.clone()
        }
        steps +=1;
        
        if temp_navigation.is_empty() && start != "ZZZ" {
            temp_navigation.extend(navigation.clone());
        }

        
    }
    println!("Part 1 answer is {}", steps);

}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}