use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;


#[allow(dead_code)]
fn part1() {
    let mut seeds: Vec<i64> = Vec::new();
    let mut almanac: HashMap<String, Vec<(i64, i64, i64)>> = HashMap::new();
    let mut seeds_maps_names: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("./inputs/input_day5.txt") {
        let mut entry_key = String::from("");
        for line in lines {
            if let Ok(line) = line {
                let clean_line = line.trim();
                if clean_line != ""{
                    if clean_line.contains(":") && !clean_line.contains("map") {
                        seeds = clean_line.split(":").nth(1).unwrap().trim().split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect();
                        continue;
                    }
                    if clean_line.contains("map:"){
                        entry_key = clean_line.trim_end_matches(":").to_string();
                        almanac.entry(entry_key.clone()).or_insert(Vec::new());
                        seeds_maps_names.push(entry_key.clone());
                        continue;
                    }
                    let mut number_iterator = clean_line.trim().split_whitespace().into_iter();
                    let destination_range_start = number_iterator.next().unwrap().parse::<i64>().unwrap();
                    let source_range_start = number_iterator.next().unwrap().parse::<i64>().unwrap();
                    let range_length = number_iterator.next().unwrap().parse::<i64>().unwrap();

                    //println!("{}, {}, {}", destination_range_start, source_range_start, range_length);
                    almanac.get_mut(&entry_key).unwrap().push((source_range_start, destination_range_start, range_length));

                    
                }

 
            }
        }

    }
    let mut locations: Vec<i64> = Vec::new();
    for seed in seeds {
        let mut lookup = seed;
        //println!("{}", seed);
        for key in &seeds_maps_names {
            for pairs in almanac.get(key).unwrap(){
                if lookup >= pairs.0 && lookup < pairs.0 + pairs.2 {
                    let diff = pairs.1 - pairs.0;
                    lookup = lookup + diff;
                    break;
                }
            }
        }
        locations.push(lookup);
    }
    //println!("{:?}", almanac);
    if let Some(min_value) = locations.iter().min() {
        println!("The minimum value is: {}", min_value);
    } else {
        println!("The vector is empty");

    }



}

#[allow(unused)]
fn main() {
    let mut seeds: Vec<i64> = Vec::new();
    let mut almanac: HashMap<String, Vec<(i64, i64, i64)>> = HashMap::new();
    let mut seeds_maps_names: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("./inputs/input_day5.txt") {
        let mut entry_key = String::from("");
        for line in lines {
            if let Ok(line) = line {
                let clean_line = line.trim();
                if clean_line != ""{
                    if clean_line.contains(":") && !clean_line.contains("map") {
                        seeds = clean_line.split(":").nth(1).unwrap().trim().split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect();
                        continue;
                    }
                    if clean_line.contains("map:"){
                        entry_key = clean_line.trim_end_matches(":").to_string();
                        almanac.entry(entry_key.clone()).or_insert(Vec::new());
                        seeds_maps_names.push(entry_key.clone());
                        continue;
                    }
                    let mut number_iterator = clean_line.trim().split_whitespace().into_iter();
                    let destination_range_start = number_iterator.next().unwrap().parse::<i64>().unwrap();
                    let source_range_start = number_iterator.next().unwrap().parse::<i64>().unwrap();
                    let range_length = number_iterator.next().unwrap().parse::<i64>().unwrap();

                    //println!("{}, {}, {}", destination_range_start, source_range_start, range_length);
                    almanac.get_mut(&entry_key).unwrap().push((source_range_start, destination_range_start, range_length));

                    
                }

 
            }
        }

    }
    let mut locations: Vec<i64> = Vec::new();
        // Iterate over pairs of values
    // for chunk in seeds.chunks_exact(2) {
    //     if let [start, length] = chunk {
    //         let start = *start;
    //         let length = *length;

    //         // Iterate over the range defined by the pair
    //         for seed in start..start + length {
    //             let mut lookup = seed;
    //             for key in &seeds_maps_names {
    //                 for pairs in almanac.get(key).unwrap(){
    //                     if lookup >= pairs.0 && lookup < pairs.0 + pairs.2 {
    //                         let diff = pairs.1 - pairs.0;
    //                         lookup = lookup + diff;
    //                         break;
    //                     }
    //                 }
    //             }
    //             locations.push(lookup);
                
    //         }
    //     }
    // }

    for i in (0..seeds.len()).step_by(2) {
        //lowest seeds[i]
        //highest seeds[i] + (seeds[i+1] - 1)
        let arr = [seeds[i], seeds[i] + (seeds[i+1] - 1)];

        let mut temp_vec: Vec<i64> = Vec::new();
        for seed in arr {
            let mut lookup = seed;
            //println!("{}", seed);
            for key in &seeds_maps_names {
                for pairs in almanac.get(key).unwrap(){
                    if lookup >= pairs.0 && lookup < pairs.0 + pairs.2 {
                        let diff = pairs.1 - pairs.0;
                        lookup = lookup + diff;
                        break;
                    }
                }
            }
            temp_vec.push(lookup);

        }

        println!("[{}, {}] -> {:?}", arr[0], arr[1], temp_vec);


        















        // for seed in seeds[i]..seeds[i+1] {
        //     println!("{}", i);
        //     let mut lookup = seed;
        //     println!("{}", seed);
        //     for key in &seeds_maps_names {
        //         for pairs in almanac.get(key).unwrap(){
        //             if lookup >= pairs.0 && lookup < pairs.0 + pairs.2 {
        //                 let diff = pairs.1 - pairs.0;
        //                 lookup = lookup + diff;
        //                 break;
        //             }
        //         }
        //     }
        //     locations.push(lookup);

        // }
    }

    //println!("{:?}", almanac);
    if let Some(min_value) = locations.iter().min() {
        println!("The minimum value is: {}", min_value);
    } else {
        println!("The vector is empty");
    }



}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}