use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[allow(dead_code)]
fn part1()->i32 {
    let mut sum = 0;

    if let Ok(lines) = read_lines("./inputs/input_day2.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(game_line) = line {
                let mut parts = game_line.split(":");
                let game_id = parts.next().unwrap().trim().split(" ").nth(1).unwrap().parse::<i32>().unwrap();
                let sets_string = parts.next().unwrap().trim().split(";");
                let mut pass = true;
                for set in sets_string {
                    let mut total_red = 0;
                    let mut total_blue = 0;
                    let mut total_green = 0;
                    let pairs: Vec<&str> = set.trim().split(", ").collect();
                    for pair in pairs {
                        let mut cubes = pair.split(" ");
                        let number = cubes.next().unwrap().trim().parse::<i32>().unwrap();
                        let color = cubes.next().unwrap().trim();
                        match color {
                            "red" => total_red += number,
                            "blue" => total_blue += number,
                            "green" => total_green += number,
                            _ => (),
                        }
                    }
                    
                    if total_red > 12 || total_green > 13 || total_blue > 14 {
                        pass = false;
                    }

                }
                if pass {
                    sum+=game_id;
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

    if let Ok(lines) = read_lines("./inputs/input_day2.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {


            let mut game_id = 0;
            let mut max_red = 0;
            let mut max_blue = 0;
            let mut max_green = 0;
    
            if let Ok(game_line) = line {
                let mut parts = game_line.split(":");
                let game_id = parts.next().unwrap().trim().split(" ").nth(1).unwrap().parse::<i32>().unwrap();
                let sets_string = parts.next().unwrap().trim().split(";");
                for set in sets_string {
                    let pairs: Vec<&str> = set.trim().split(", ").collect();
                    for pair in pairs {
                        let mut cubes = pair.split(" ");
                        let number = cubes.next().unwrap().trim().parse::<i32>().unwrap();
                        let color = cubes.next().unwrap().trim();
                        match color {
                            "red" => if number > max_red { max_red = number },
                            "blue" => if number > max_blue { max_blue = number },
                            "green" => if number > max_green { max_green = number },
                            _ => (),
                        }
                    }
                }
                let power = max_red * max_green * max_blue;
                sum += power;


                
            }
        }
    }
    println!("Sum for part2 is {}", sum);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}