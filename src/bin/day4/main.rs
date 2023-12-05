use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


#[allow(dead_code)]
fn part1() -> i32 {
    let mut sum = 0;
    
    if let Ok(lines) = read_lines("./inputs/input_day4.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(game_line) = line {
                let mut parts = game_line.split(":");
                //let card_id = parts.next().unwrap().trim_start_matches("Card").trim().parse::<i32>().unwrap();
                //println!("{:?}", card_id);
                let mut sets_cards = parts.next().unwrap().trim().split(" | ");
                let winning_numbers: Vec<i32> = sets_cards.next().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
                let your_numbers: Vec<i32> = sets_cards.next().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
                let your_winnings: Vec<i32> = winning_numbers.iter().filter(|&x| your_numbers.contains(x)).cloned().collect();
                if let Some(e) = (your_winnings.len() as u32).checked_sub(1){
                    sum += i32::pow(2, e);
                } 
            }
        }
        println!("Sum is {}", sum);
    }
    sum

}

#[allow(unused)]
fn main() {
    let mut sum = 0;
    
    if let Ok(lines) = read_lines("./inputs/input_day4.txt") {
        // Open the file
        let file = File::open("./inputs/input_day4.txt").unwrap();
        let reader = io::BufReader::new(file);

        // Count the lines
        let line_count = reader.lines().count();
        // Consumes the iterator, returns an (Optional) String
        let mut cards: Vec<i32> = vec![1; line_count];
        for line in lines {
            if let Ok(game_line) = line {
                let mut parts = game_line.split(":");
                let mut card_id = parts.next().unwrap().trim_start_matches("Card").trim().parse::<i32>().unwrap();
                //println!("{:?}", card_id);
                let mut sets_cards = parts.next().unwrap().trim().split(" | ");
                let winning_numbers: Vec<i32> = sets_cards.next().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
                let your_numbers: Vec<i32> = sets_cards.next().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
                let your_winnings: Vec<i32> = winning_numbers.iter().filter(|&x| your_numbers.contains(x)).cloned().collect();

                for num_copy in 0..cards[card_id as usize - 1] {
                    let mut id_copy = card_id.clone();
                    for i in your_winnings.clone().into_iter(){
                        id_copy += 1;
                        cards[id_copy as usize - 1] += 1;
                    }
                }
 
            }
        }
        sum = cards.iter().sum();

        println!("Sum is {}", sum);
    }


}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}