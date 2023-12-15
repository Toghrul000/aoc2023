use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn transpose_vec_string(v: Vec<String>) -> Vec<String> {
    return transpose(v.iter().map(|s| s.chars().collect()).collect())
    .iter().map(|char_vec| char_vec.iter().collect()).collect();
}

fn tilt(platform: &mut Vec<String>) {
    platform.iter_mut().for_each(|s| {
        let mut chars_s: Vec<char> = s.chars().collect();

        let mut index_to_add = 0;
        for i in 0..chars_s.len() {
            if chars_s[i] == '.' && chars_s[index_to_add] != '.' {
                index_to_add = i;
            }

            if chars_s[i] == '#' {
                if let Some(i_next) = i.checked_add(1) {
                    index_to_add = i_next;
                }
            }

            if chars_s[i] == 'O' && chars_s[index_to_add] != 'O' {
                chars_s[index_to_add] = chars_s[i];
                chars_s[i] = '.';
                if index_to_add + 1 < chars_s.len() {
                    index_to_add += 1;
                }
            }
        }
        let n_s: String = chars_s.iter().collect();
        *s = n_s;
    });
}



#[allow(unused)]
pub fn main() {

    let mut platform: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("./inputs/input_day14.txt") {
        for line in lines {
            if let Ok(line) = line {
                //println!("{}", line);
                platform.push(line);
            } 
        }
    }
    //println!();

    platform = transpose_vec_string(platform);


    // platform.iter().for_each(|s| {
    //     println!("{}", s);
    // });

    // println!();
    tilt(&mut platform);

    platform = transpose_vec_string(platform);

    let mut sum = 0;
    let mut row_num = platform.len();
    platform.iter().for_each(|s| {
        let c = s.chars().filter(|&c| c == 'O').count();
        sum += c * row_num;
        row_num-=1;
    });
    
    println!("Part 1: {}", sum);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}