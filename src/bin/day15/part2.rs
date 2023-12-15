use std::collections::HashMap;
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

    let mut label_value:HashMap<String, u32>  = HashMap::new();
    for s in init_sequence.clone() {
        let mut current_value = 0;
        let s1;
        if s.contains("=") {
            s1 = s.split("=").next().unwrap().to_string();
        } else {
            s1 = s.split("-").next().unwrap().to_string();
        }
       
        for c in s1.chars(){
            current_value += c as u32;
            current_value = current_value * 17;
            current_value = current_value % 256;
    
        }
        //println!("{} and {}", s1, current_value);
        label_value.insert(s1, current_value);

    }

    let mut boxes: HashMap<u32, Vec<(String, u32)>> = HashMap::new();

    for s in init_sequence {
        if s.contains("=") {
            let mut parts = s.split("=");
            let label = parts.next().unwrap().to_string();
            let focal_lenght = parts.next().unwrap().parse::<u32>().unwrap();

            if boxes.contains_key(&label_value[&label]){

                let mut vec = boxes.get_mut(&label_value[&label]).unwrap();
                if let Some(index) = vec.iter().position(|(key, _)| *key == label) {
                    vec.remove(index);
                    vec.insert(index, (label, focal_lenght));
                } else {
                    boxes.get_mut(&label_value[&label]).unwrap().push((label, focal_lenght));
                }
                
            } else {
                let item: Vec<(String, u32)> = Vec::new();
                boxes.insert(label_value[&label], item);
                boxes.get_mut(&label_value[&label]).unwrap().push((label, focal_lenght));
            }
            //println!("{:?}", boxes);
            
        } else {
            let label = s.split("-").next().unwrap().to_string();
            if boxes.contains_key(&label_value[&label]){
                let mut vec = boxes.get_mut(&label_value[&label]).unwrap();
                if let Some(index) = vec.iter().position(|(key, _)| *key == label) {
                    vec.remove(index);
                }
            }
            //println!("{:?}", boxes);
        }

    }

    let mut sum = 0;
    for (bnum, b_vec) in boxes {
        for (i, (label, focal_length)) in b_vec.iter().enumerate() {
            //println!("Box {}, slot {}, focal {}", (bnum + 1), (i as u32 + 1), focal_length);
            sum += (bnum + 1) * (i as u32 + 1) * focal_length;
        }
    }

    println!("Part 2 {}", sum);


}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}