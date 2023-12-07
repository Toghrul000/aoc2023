
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

#[derive(Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}





fn classify_hand(hand: &str) -> HandType {
    let mut card_counts: HashMap<char, usize> = HashMap::new();

    let mut jokers = 0;
    for card in hand.chars() {
        if card == 'J' {
            jokers+=1;
        }
        *card_counts.entry(card).or_insert(0) += 1;
    }
    if jokers > 0 && jokers < 5 {
        card_counts.remove(&'J');
        if let Some((&max_val_key, _)) = card_counts.iter().max_by_key(|&(_, value)| value) {
            *card_counts.get_mut(&max_val_key).unwrap() += jokers;
        }
    }
    let mut distinct_counts: Vec<usize> = card_counts.values().cloned().collect();
    distinct_counts.sort_by(|a, b| b.cmp(a));
    

    match distinct_counts.as_slice() {
        [5] => HandType::FiveOfAKind,
        [4, 1] => HandType::FourOfAKind,
        [3, 2] => HandType::FullHouse,
        [3, 1, 1] => HandType::ThreeOfAKind,
        [2, 2, 1] => HandType::TwoPair,
        [2, 1, 1, 1] => HandType::OnePair,
        _ => HandType::HighCard,
    }
}

fn compare_strings(s1: &str, s2: &str) -> std::cmp::Ordering {
    let order = "AKQT98765432J";

    for (ch1, ch2) in s1.chars().zip(s2.chars()) {
        let strength1 = order.find(ch1).unwrap_or(order.len());
        let strength2 = order.find(ch2).unwrap_or(order.len());

        match strength1.cmp(&strength2) {
            std::cmp::Ordering::Equal => continue,
            result => return result,
        }
    }

    // If the loop completes, it means the strings have the same prefix.
    // The longer string is considered greater.
    s1.len().cmp(&s2.len())
}

fn assign_ranks(cards: &mut Vec<(String, i32)>, rank: &mut i32) -> Vec<(String, i32, i32)> {
    cards.sort_by(|a, b| compare_strings(b.0.as_str(), a.0.as_str()));

    let mut result: Vec<(String, i32, i32)> = Vec::new();
    let mut prev_str = "";

    for (current_str, i) in cards.iter() {
        if prev_str.is_empty() || compare_strings(prev_str, current_str) != std::cmp::Ordering::Equal {
            // Different prefix, increment rank
            *rank += 1;
        }
        // Assign the rank to the current string and store in the result vector
        result.push((current_str.clone(), *i, *rank));
        prev_str = current_str;
    }

    result
}


#[allow(unused)]
pub fn main() {

    let mut ranks_cards: HashMap<HandType, Vec<(String, i32)>> = HashMap::new();
    let types_ordered: Vec<HandType> = vec![
        HandType::HighCard,
        HandType::OnePair,
        HandType::TwoPair,
        HandType::ThreeOfAKind,
        HandType::FullHouse,
        HandType::FourOfAKind,
        HandType::FiveOfAKind,
    ];

    if let Ok(lines) = read_lines("./inputs/input_day7.txt") {
        for line in lines {
            if let Ok(line) = line {
                let card = line.trim().split_whitespace().next().unwrap().to_string();           
                let bid =  line.trim().split_whitespace().nth(1).unwrap().parse::<i32>().unwrap();
                let hand_type = classify_hand(card.as_str());
                //println!("{:?}", hand_type);

                ranks_cards
                .entry(hand_type)
                .or_insert_with(Vec::new)
                .push((card, bid));
            } 
        }

    }

    let mut rank = 0;
    let mut sum = 0;
    for t in types_ordered {
        if let Some(pairs) = ranks_cards.get_mut(&t){
            //println!("{:?}", t);
            let ranked_cards: Vec<(String, i32, i32)> = assign_ranks(pairs, &mut rank);
            //println!("{:?}", ranked_cards);
            for (card, bid, rank) in ranked_cards {
                sum+= bid * rank; 
            }

        }

    }

    println!("Part 2 answer: {}", sum);


}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}