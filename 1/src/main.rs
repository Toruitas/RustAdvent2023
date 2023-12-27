use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    let mut final_calibration:u32 = 0;

    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(mixed_coordinate) = line {
                final_calibration += sum_digits(mixed_coordinate);
            }
        }
    }

    println!("{}", final_calibration);
}

fn is_string_numeric(str: &str) -> bool {
    for c in str.chars() {
        if c.is_alphabetic() {
            return false;
        }
    }
    return true;
}

fn sum_digits(mixed_coordinate: String) -> u32{
    let mut str_candidates:Vec<(u32, &str)> = Vec::new();
    let mut digit_candidates:Vec<u32> = Vec::new();

    println!("{}", &mixed_coordinate);
    let find_vals= [
        "one", "1",
        "two", "2",
        "three", "3",
        "four", "4",
        "five", "5",
        "six", "6",
        "seven", "7",
        "eight", "8",
        "nine", "9",
        // "zero", "0",
    ];    

    // for repl in find_vals.iter() {
    //     let mixed_coordinate_pos = mixed_coordinate.find(repl);  // finds first instance only
    //     match mixed_coordinate_pos {
    //         Some(pos) => {
    //             str_candidates.push((pos as u32, repl));
    //         },
    //         None => {}
    //     }
    // }

    for repl in find_vals.iter() {
        let mixed_coordinate_pos: Vec<_> = mixed_coordinate.match_indices(repl).collect();
        for t in mixed_coordinate_pos.iter(){
            str_candidates.push((t.0 as u32, t.1));
        }
    }

    // sort positions
    str_candidates.sort_by(|a, b| a.0.cmp(&b.0));
    println!("{:?}", str_candidates);

    let mut replacements = HashMap::new();
    replacements.insert(String::from("one"), 1);
    replacements.insert(String::from("two"), 2);
    replacements.insert(String::from("three"), 3);
    replacements.insert(String::from("four"), 4);
    replacements.insert(String::from("five"), 5);
    replacements.insert(String::from("six"), 6);
    replacements.insert(String::from("seven"), 7);
    replacements.insert(String::from("eight"), 8);
    replacements.insert(String::from("nine"), 9);
    // replacements.insert(String::from("zero"), 0);

    for v in str_candidates.iter() {
        if is_string_numeric(v.1) {
            digit_candidates.push(v.1.parse::<u32>().unwrap());
        }
        else{
            digit_candidates.push(replacements.get(v.1).copied().unwrap_or(0));
        }
    }
    println!("{:?}", digit_candidates);

    // let replace_vals = [
    //     ("one", 1),
    //     ("two", 2),
    //     ("three", 3),
    //     ("four", 4),
    //     ("five", 5),
    //     ("six", 6),
    //     ("seven", 7),
    //     ("eight", 8),
    //     ("nine", 9),
    //     ("zero", 0),
    // ];

    // for v in str_candidates.iter() {
    //     if v.1.is_digit(10) {
    //         digit_candidates.push(v.1.to_digit(10).unwrap());
    //     } else{
    //         for repl_val in replace_vals{
    //             v.1.replace(repl_val.0, repl_val.1);
    //         }
            
    //     }
    
    // }



    println!("{}", &mixed_coordinate);

    // for char in mixed_coordinate.chars() {
        
    // }
    // println!("{:?}", &digit_candidates);
    // println!("{:?}", &digit_candidates[0]);
    // println!("{:?}", &digit_candidates[digit_candidates.len()-1]);
    // println!("{:?}", digit_candidates.len());

    let calculated_coord:u32;

    if digit_candidates.len() == 1{
        calculated_coord = digit_candidates[0] * 11;
    }else{
        calculated_coord = &digit_candidates[0] * 10 + &digit_candidates[digit_candidates.len()-1];
    }

    println!("{:?}", calculated_coord);
    calculated_coord

}

// from https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html  
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}