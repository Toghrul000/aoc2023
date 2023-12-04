
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, PartialEq)]
struct Coordinates {
    x: usize,
    y: usize,
 }

 #[derive(Debug, PartialEq)]
 struct Number {
    val: String,
    surrounding_coordinates: Vec<Coordinates>,
 }

 #[derive(Debug, PartialEq)]
 struct Star {
    surrounding_coordinates: Vec<Coordinates>,
 }

 #[allow(dead_code)]
 fn part1() -> i32 {

    let mut sum = 0;
    let mut data: Vec<Vec<char>> = Vec::new();


    if let Ok(lines) = read_lines("./inputs/input_day3.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                //println!("{}", line);
                let mut row = Vec::new();
                for c in line.chars(){
                    row.push(c);

                }
                data.push(row);
            }
        }

        let mut numbers: Vec<Number> = Vec::new();
        for (i, row) in data.iter().enumerate() {
            let mut current_number = Number {
                val: String::new(),
                surrounding_coordinates: Vec::new(),
            };
    
            for (j, &ch) in row.iter().enumerate() {
                if ch.is_digit(10) {
                    current_number.val.push(ch);

                    // Top left
                    if let (Some(new_i), Some(new_j)) = (i.checked_sub(1), j.checked_sub(1)) {
                        if new_i < data.len().try_into().unwrap() && new_j < row.len().try_into().unwrap() 
                        && !data[new_i as usize][new_j as usize].is_digit(10) {
                            //println!("({}, {}) is a valid coordinate", new_i, new_j);
                            let coordinate = Coordinates {y: new_i, x: new_j};
                            if !current_number.surrounding_coordinates.contains(&coordinate){
                                current_number.surrounding_coordinates.push(coordinate);
                            }
                        }
                    }

                    // Top center
                    if let (Some(new_i), Some(new_j)) = (i.checked_sub(1), j.checked_sub(0)) {
                        if new_i < data.len().try_into().unwrap() && new_j < row.len().try_into().unwrap() && !data[new_i as usize][new_j as usize].is_digit(10) {
                            //println!("({}, {}) is a valid coordinate", new_i, new_j);
                            let coordinate = Coordinates {y: new_i, x: new_j};
                            if !current_number.surrounding_coordinates.contains(&coordinate){
                                current_number.surrounding_coordinates.push(coordinate);
                            }
                        }
                    }

                    // Top right
                    if let (Some(new_i), Some(new_j)) = (i.checked_sub(1), j.checked_add(1)) {
                        if new_i < data.len().try_into().unwrap() && new_j < row.len().try_into().unwrap() && !data[new_i as usize][new_j as usize].is_digit(10) {
                            //println!("({}, {}) is a valid coordinate", new_i, new_j);
                            let coordinate = Coordinates {y: new_i, x: new_j};
                            if !current_number.surrounding_coordinates.contains(&coordinate){
                                current_number.surrounding_coordinates.push(coordinate);
                            }
                        }
                    }

                    // Center left
                    if let (Some(new_i), Some(new_j)) = (i.checked_sub(0), j.checked_sub(1)) {
                        if new_i < data.len().try_into().unwrap() && new_j < row.len().try_into().unwrap() && !data[new_i as usize][new_j as usize].is_digit(10) {
                            //println!("({}, {}) is a valid coordinate", new_i, new_j);
                            let coordinate = Coordinates {y: new_i, x: new_j};
                            if !current_number.surrounding_coordinates.contains(&coordinate){
                                current_number.surrounding_coordinates.push(coordinate);
                            }
                        }
                    }

                    // Center right
                    if let (Some(new_i), Some(new_j)) = (i.checked_sub(0), j.checked_add(1)) {
                        if new_i < data.len().try_into().unwrap() && new_j < row.len().try_into().unwrap() && !data[new_i as usize][new_j as usize].is_digit(10) {
                            //println!("({}, {}) is a valid coordinate", new_i, new_j);
                            let coordinate = Coordinates {y: new_i, x: new_j};
                            if !current_number.surrounding_coordinates.contains(&coordinate){
                                current_number.surrounding_coordinates.push(coordinate);
                            }
                        }
                    }

                    // Bottom left
                    if let (Some(new_i), Some(new_j)) = (i.checked_add(1), j.checked_sub(1)) {
                        if new_i < data.len().try_into().unwrap() && new_j < row.len().try_into().unwrap() && !data[new_i as usize][new_j as usize].is_digit(10) {
                            //println!("({}, {}) is a valid coordinate", new_i, new_j);
                            let coordinate = Coordinates {y: new_i, x: new_j};
                            if !current_number.surrounding_coordinates.contains(&coordinate){
                                current_number.surrounding_coordinates.push(coordinate);
                            }
                        }
                    }

                    // Bottom center
                    if let (Some(new_i), Some(new_j)) = (i.checked_add(1), j.checked_sub(0)) {
                        if new_i < data.len().try_into().unwrap() && new_j < row.len().try_into().unwrap() && !data[new_i as usize][new_j as usize].is_digit(10) {
                            //println!("({}, {}) is a valid coordinate", new_i, new_j);
                            let coordinate = Coordinates {y: new_i, x: new_j};
                            if !current_number.surrounding_coordinates.contains(&coordinate){
                                current_number.surrounding_coordinates.push(coordinate);
                            }
                        }
                    }

                    // Bottom right
                    if let (Some(new_i), Some(new_j)) = (i.checked_add(1), j.checked_add(1)) {
                        if new_i < data.len().try_into().unwrap() && new_j < row.len().try_into().unwrap() && !data[new_i as usize][new_j as usize].is_digit(10) {
                            //println!("({}, {}) is a valid coordinate", new_i, new_j);
                            let coordinate = Coordinates {y: new_i, x: new_j};
                            if !current_number.surrounding_coordinates.contains(&coordinate){
                                current_number.surrounding_coordinates.push(coordinate);
                            }
                        }
                    }
     
    
                    // Check the next character
                    if j + 1 < row.len() && row[j + 1].is_digit(10) {
                        continue; // Continue collecting digits for the current number
                    }
    
                    // Print or process the collected number
                    if !current_number.val.is_empty() {
                        //println!("Number in row {}: {}", i + 1, current_number.val);
                        //println!("{:?}", current_number);
                        numbers.push(current_number);
                        current_number = Number {
                            val: String::new(),
                            surrounding_coordinates: Vec::new(),
                        };
                    }
                }
            }
        }

        for number in numbers {
            for coordinate in number.surrounding_coordinates{
                if data[coordinate.y][coordinate.x] != '.'{

                    sum += number.val.parse::<i32>().unwrap();
                    break;
                }
            }
        }

    }
    println!("Sum is {}", sum);
    sum


 }

#[allow(unused)]
fn main() {

    let mut sum = 0;
    let mut data: Vec<Vec<char>> = Vec::new();

    if let Ok(lines) = read_lines("./inputs/input_day3.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                //println!("{}", line);
                let mut row = Vec::new();
                for c in line.chars(){
                    row.push(c);

                }
                data.push(row);
            }
        }

        let mut numbers: Vec<Number> = Vec::new();
        for (i, row) in data.iter().enumerate() {
            let mut current_number = Number {
                val: String::new(),
                surrounding_coordinates: Vec::new(),
            };
    
            for (j, &ch) in row.iter().enumerate() {
                if ch.is_digit(10) {
                    current_number.val.push(ch);

                    let coordinate = Coordinates {y: i, x: j};
                    if !current_number.surrounding_coordinates.contains(&coordinate){
                        current_number.surrounding_coordinates.push(coordinate);
                    }

     
    
                    // Check the next character
                    if j + 1 < row.len() && row[j + 1].is_digit(10) {
                        continue; // Continue collecting digits for the current number
                    }
    
                    // Print or process the collected number
                    if !current_number.val.is_empty() {
                        //println!("Number in row {}: {}", i + 1, current_number.val);
                        //println!("{:?}", current_number);
                        numbers.push(current_number);
                        current_number = Number {
                            val: String::new(),
                            surrounding_coordinates: Vec::new(),
                        };
                    }
                }
            }
        }

        let mut stars: Vec<Star> = Vec::new();
        for (i, row) in data.iter().enumerate() {
            for (j, &ch) in row.iter().enumerate() {
                if ch == '*' {
                    let mut current_star = Star {
                        surrounding_coordinates: Vec::new(),
                    };


                    // Top left
                    if let (Some(new_i), Some(new_j)) = (i.checked_sub(1), j.checked_sub(1)) {
                        if new_i < data.len().try_into().unwrap() && new_j < row.len().try_into().unwrap() 
                        && data[new_i as usize][new_j as usize].is_digit(10) {
                            //println!("({}, {}) is a valid coordinate", new_i, new_j);
                            let coordinate = Coordinates {y: new_i, x: new_j};
                            if !current_star.surrounding_coordinates.contains(&coordinate){
                                current_star.surrounding_coordinates.push(coordinate);
                            }
                        }
                    }

                    // Top center
                    if let (Some(new_i), Some(new_j)) = (i.checked_sub(1), j.checked_sub(0)) {
                        if new_i < data.len().try_into().unwrap() && new_j < row.len().try_into().unwrap() 
                        && data[new_i as usize][new_j as usize].is_digit(10) {
                            //println!("({}, {}) is a valid coordinate", new_i, new_j);
                            let coordinate = Coordinates {y: new_i, x: new_j};
                            if !current_star.surrounding_coordinates.contains(&coordinate){
                                current_star.surrounding_coordinates.push(coordinate);
                            }
                        }
                    }

                    // Top right
                    if let (Some(new_i), Some(new_j)) = (i.checked_sub(1), j.checked_add(1)) {
                        if new_i < data.len().try_into().unwrap() && new_j < row.len().try_into().unwrap() 
                        && data[new_i as usize][new_j as usize].is_digit(10) {
                            //println!("({}, {}) is a valid coordinate", new_i, new_j);
                            let coordinate = Coordinates {y: new_i, x: new_j};
                            if !current_star.surrounding_coordinates.contains(&coordinate){
                                current_star.surrounding_coordinates.push(coordinate);
                            }
                        }
                    }

                    // Center left
                    if let (Some(new_i), Some(new_j)) = (i.checked_sub(0), j.checked_sub(1)) {
                        if new_i < data.len().try_into().unwrap() && new_j < row.len().try_into().unwrap() 
                        && data[new_i as usize][new_j as usize].is_digit(10) {
                            //println!("({}, {}) is a valid coordinate", new_i, new_j);
                            let coordinate = Coordinates {y: new_i, x: new_j};
                            if !current_star.surrounding_coordinates.contains(&coordinate){
                                current_star.surrounding_coordinates.push(coordinate);
                            }
                        }
                    }

                    // Center right
                    if let (Some(new_i), Some(new_j)) = (i.checked_sub(0), j.checked_add(1)) {
                        if new_i < data.len().try_into().unwrap() && new_j < row.len().try_into().unwrap() 
                        && data[new_i as usize][new_j as usize].is_digit(10) {
                            //println!("({}, {}) is a valid coordinate", new_i, new_j);
                            let coordinate = Coordinates {y: new_i, x: new_j};
                            if !current_star.surrounding_coordinates.contains(&coordinate){
                                current_star.surrounding_coordinates.push(coordinate);
                            }
                        }
                    }

                    // Bottom left
                    if let (Some(new_i), Some(new_j)) = (i.checked_add(1), j.checked_sub(1)) {
                        if new_i < data.len().try_into().unwrap() && new_j < row.len().try_into().unwrap() 
                        && data[new_i as usize][new_j as usize].is_digit(10) {
                            //println!("({}, {}) is a valid coordinate", new_i, new_j);
                            let coordinate = Coordinates {y: new_i, x: new_j};
                            if !current_star.surrounding_coordinates.contains(&coordinate){
                                current_star.surrounding_coordinates.push(coordinate);
                            }
                        }
                    }

                    // Bottom center
                    if let (Some(new_i), Some(new_j)) = (i.checked_add(1), j.checked_sub(0)) {
                        if new_i < data.len().try_into().unwrap() && new_j < row.len().try_into().unwrap() 
                        && data[new_i as usize][new_j as usize].is_digit(10) {
                            //println!("({}, {}) is a valid coordinate", new_i, new_j);
                            let coordinate = Coordinates {y: new_i, x: new_j};
                            if !current_star.surrounding_coordinates.contains(&coordinate){
                                current_star.surrounding_coordinates.push(coordinate);
                            }
                        }
                    }

                    // Bottom right
                    if let (Some(new_i), Some(new_j)) = (i.checked_add(1), j.checked_add(1)) {
                        if new_i < data.len().try_into().unwrap() && new_j < row.len().try_into().unwrap() 
                        && data[new_i as usize][new_j as usize].is_digit(10) {
                            //println!("({}, {}) is a valid coordinate", new_i, new_j);
                            let coordinate = Coordinates {y: new_i, x: new_j};
                            if !current_star.surrounding_coordinates.contains(&coordinate){
                                current_star.surrounding_coordinates.push(coordinate);
                            }
                        }
                    }

                    stars.push(current_star);
                }
            }
        }

        for star in stars {
            let mut c  = 0;
            let mut adjacent_numbers:Vec<&Number> = Vec::new();
            let mut product = 0;
            for number in &numbers {
                let have_common_element = star.surrounding_coordinates.iter().any(|item| number.surrounding_coordinates.contains(item));
                if have_common_element {
                    adjacent_numbers.push(number);

                }
            }

            if adjacent_numbers.len() == 2 {
                // println!("{:?}", adjacent_numbers);
                product = adjacent_numbers[0].val.parse::<i32>().unwrap() * adjacent_numbers[1].val.parse::<i32>().unwrap();
                sum += product;
                
            }

        }




    }
    println!("Sum is {}", sum);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}