use std::cmp::{min, max};
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


fn apply_range(map: Vec<(i64, i64, i64)>,  mut R: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let mut A: Vec<(i64, i64)> = Vec::new();
    for (src, dest, range_length) in map {
        let src_end = src + range_length;
        let mut NR: Vec<(i64, i64)> = Vec::new();

        while R.len() != 0 {
            // [start_seed                                     end_seed)
            //                 [src       src_end]
            // [BEFORE        ][INTER            ][AFTER        )
            let (start_seed, end_seed) = R.pop().unwrap();
            let before: (i64, i64) = (start_seed, min(end_seed,src));
            let inter: (i64, i64) = (max(start_seed, src), min(src_end, end_seed));
            let after: (i64, i64) = (max(src_end, start_seed), end_seed);

            if before.1 > before.0{
                NR.push(before);
            }
            if inter.1 > inter.0 {
                A.push((inter.0-src+dest, inter.1-src+dest));
            }

            if after.1 > after.0 {
                NR.push(after);

            }    
        }
        R = NR.clone();
        
        
    }
    A.extend(R.clone());
    A
    
    
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
    for i in (0..seeds.len()).step_by(2) {
        let mut R: Vec<(i64, i64)> = vec![(seeds[i], seeds[i] + seeds[i+1])];

        for key in &seeds_maps_names {
            R = apply_range(almanac.get(key).unwrap().clone(), R);
        }
        locations.push(R.iter().min().unwrap().0)
    }


    ////Brute force solution
    // let mut locations: Vec<i64> = Vec::new();
    // for i in (0..seeds.len()).step_by(2) {
    //     for seed in seeds[i]..seeds[i+1] {
    //         println!("{}", i);
    //         let mut lookup = seed;
    //         println!("{}", seed);
    //         for key in &seeds_maps_names {
    //             for pairs in almanac.get(key).unwrap(){
    //                 if lookup >= pairs.0 && lookup < pairs.0 + pairs.2 {
    //                     let diff = pairs.1 - pairs.0;
    //                     lookup = lookup + diff;
    //                     break;
    //                 }
    //             }
    //         }
    //         locations.push(lookup);

    //     }
    // }

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