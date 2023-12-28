use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn main() {
    let mut final_answer:i32 = 0;

    let mut encoded_table:Vec<Vec<i32>> = Vec::new();
    let mut symbol_table:Vec<Vec<i32>> = Vec::new();
    let mut zero_row = Vec::new();

    let mut init_zeroes = false;

    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String

        // let mut prev_row_encoded:Vec<i32> = Vec::new();

        for line in lines {
            let mut start_idx:Option<usize> = None;
            let mut encoded_row:Vec<i32> = Vec::from([0]); // left pad
            let mut symbol_row:Vec<i32> = Vec::from([0]);

            if let Ok(row_string) = line {
                // add first vecs
                if !init_zeroes {
                    for i in 0..row_string.len() + 2 {
                        zero_row.push(0);
                    }
                    encoded_table.push(zero_row.clone());
                    symbol_table.push(zero_row.clone());
                    init_zeroes = true;
                }
                
                for (idx, c) in row_string.trim().chars().enumerate() {
                    if c == '.'{
                        // if we have a start idx we have to close out the number
                        match start_idx {
                            None => {},
                            Some(mut start_i) => {
                                let parsed_val = row_string[start_i..idx].parse::<i32>().unwrap();

                                while start_i < idx {
                                    encoded_row.push(parsed_val);
                                    start_i +=1;
                                }

                                // reset for next one
                                start_idx = None;
                            }
                        }

                        // push a 0 for the .
                        encoded_row.push(0);
                        symbol_row.push(0);

                    } else if c.is_digit(10){

                        // if this is the end of the row and we have a start, insert numbers
                        // otherwise defer until we have a full num
                        match start_idx {
                            None => {
                                start_idx = Some(idx)
                            },
                            Some(mut start_i) => {
                                if idx == row_string.len() - 1 {
                                    let parsed_val = row_string[start_i..=idx].parse::<i32>().unwrap();  // note we include the final digit here

                                    while start_i < idx {
                                        encoded_row.push(parsed_val);
                                        start_i +=1;
                                    }

                                    // reset for next one
                                    start_idx = None;
                                }
                            }
                        }
                        
                        symbol_row.push(0);

                    } 
                    else if c == '*'{
                        // if we have a start idx we have to close out the number
                        match start_idx {
                            None => {},
                            Some(mut start_i) => {
                                let parsed_val = row_string[start_i..idx].parse::<i32>().unwrap();

                                while start_i < idx {
                                    encoded_row.push(parsed_val);
                                    start_i +=1;
                                }

                                // reset for next one
                                start_idx = None;
                            }
                        }

                        // push a 0 for the .
                        encoded_row.push(0);
                        symbol_row.push(1);

                    }
                    else {
                        // it is a symbol
                        // if we have a start idx we have to close out the number 
                        match start_idx {
                            None => {},
                            Some(mut start_i) => {
                                let parsed_val = row_string[start_i..idx].parse::<i32>().unwrap();

                                while start_i < idx {
                                    encoded_row.push(parsed_val);
                                    start_i +=1 ;
                                }

                                // reset for next one
                                start_idx = None;
                            }
                        }


                        // push the symbol
                        encoded_row.push(0);
                        symbol_row.push(0);
                    }
                }
                // padding
                encoded_row.push(0);
                symbol_row.push(0);
                encoded_table.push(encoded_row);
                symbol_table.push(symbol_row); 

                
            }
        
        }
        encoded_table.push(zero_row.clone());
        symbol_table.push(zero_row.clone());

        println!("{:?}", &encoded_table);
        println!("{:?}", &symbol_table);

        // now we go through the values table and expand the 1s to the 8 adjacent points in the 3 relevant rows: top, this, bottom
        // and add in where there's a 1

        for (y, row) in encoded_table.iter().enumerate() {
            for (x, val) in row.iter().enumerate() {
                if symbol_table[y][x] == 1{
                    // use the a window of sets
                    let mut top_row = HashSet::from([encoded_table[y-1][x-1]*symbol_table[y][x], encoded_table[y-1][x]*symbol_table[y][x], encoded_table[y-1][x+1]*symbol_table[y][x]]);
                    let mut mid_row = HashSet::from([encoded_table[y][x-1]*symbol_table[y][x], encoded_table[y][x]*symbol_table[y][x], encoded_table[y][x+1]*symbol_table[y][x]]);
                    let mut bottom_row = HashSet::from([encoded_table[y+1][x-1]*symbol_table[y][x], encoded_table[y+1][x]*symbol_table[y][x], encoded_table[y+1][x+1]*symbol_table[y][x]]);

                    top_row.remove(&0);
                    mid_row.remove(&0);
                    bottom_row.remove(&0);

                    println!("{:?}", &top_row);
                    println!("{:?}", &mid_row);
                    println!("{:?}", &bottom_row);
                    println!("");

                    if top_row.len() + mid_row.len() + bottom_row.len() == 2{ 
                        let mut combo:Vec<i32> = Vec::new();

                        for v in top_row.iter() {
                            combo.push(*v);
                        }

                        for v in mid_row.iter() {
                            combo.push(*v);
                        }

                        for v in bottom_row.iter() {
                            combo.push(*v);
                        }

                        println!("{:?}", &combo);
                        final_answer += combo[0]*combo[1];

                    }

                    // for n in top_row{
                    //     final_answer += n;
                    // }

                    // for n in mid_row{
                    //     final_answer += n;
                    // }

                    // for n in bottom_row{
                    //     final_answer += n;
                    // }
                }
            }
        }
        

    }

    println!("{}", final_answer);
}

// from https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html  
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
