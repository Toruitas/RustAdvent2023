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

                        // if start_idx != None {
                        //     // parse and push
                        //     let parsed_val = &line[start_idx:idx-1].parse::<u32>().unwrap();

                        //     while start_idx < idx {
                        //         encoded_row.push(parsed_val);
                        //         start_idx += 1;
                        //     }

                        //     // reset for next one
                        //     start_idx = None;
                        // }

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

                        // if start_idx {
                        //     // parse and push
                        //     let parsed_val = &line[start_idx:idx-1].parse::<u32>().unwrap();

                        //     while start_idx < idx {
                        //         encoded_row.push(parsed_val);
                        //         start_idx += 1;
                        //     }

                        //     // reset for next one
                        //     start_idx = None;
                        // }
                        // // if we don't have a start index, it's a new number
                        // else{
                        //     start_idx = idx;
                        // }
                    } else {
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

                        // if start_idx {
                        //     // parse and push
                        //     let parsed_val = &line[start_idx:idx-1].parse::<u32>().unwrap();

                        //     while start_idx < idx {
                        //         encoded_row.push(parsed_val);
                        //         start_idx += 1;
                        //     }

                        //     // reset for next one
                        //     start_idx = None;
                        // }

                        // push the symbol
                        encoded_row.push(0);
                        symbol_row.push(1);
                    }
                }
                // padding
                encoded_row.push(0);
                symbol_row.push(0);
                encoded_table.push(encoded_row);
                symbol_table.push(symbol_row); 

                // analyze this and prev row
                // match indices of -1 in both directions. 
                
                // this row's symbols against prev row's numbers and its own nums
                // let mut add_nums = HashSet::new();
                // let mut add_nums:Vec<i32> = Vec::new();
                
                // println!("{:?}", &row_string);
                // // println!("{:?}", &prev_row_encoded);
                // println!("{:?}", &encoded_row);

                // for i in 0..encoded_row.len() - 1 { 

                //     // it is a symbol, check the nums in the previous row and this row
                //     if encoded_row[i] == -1 {
                //         // prev row

                //         if prev_row_encoded.len() > 0 {
                //             let mut push_me_from_above = 0;

                //             if i > 0 && prev_row_encoded[i-1] > 0 {
                //                 push_me_from_above = prev_row_encoded[i-1];
                //                 // add_nums.push(prev_row_encoded[i-1]);
    
                //                 // remove all instances of the num from prev_row_encoded to prevent double addition
                //                 // let repl_num = &prev_row_encoded[i-1];
                //                 // prev_row_encoded = repl_num_w_zero(repl_num, &prev_row_encoded);
                //             }

                //             println!("{:?}", &push_me_from_above);
                            
                //             if prev_row_encoded[i] > 0 {
                //                 // if we don't have a push_me, it's a new num
                //                 if push_me_from_above == 0 {
                //                     push_me_from_above = prev_row_encoded[i];
                //                 }
                //                 // add_nums.push(prev_row_encoded[i]);
    
                //                 // remove all instances of the num from prev_row_encoded to prevent double addition
                //                 // let repl_num = &prev_row_encoded[i];
                //                 // prev_row_encoded = repl_num_w_zero(repl_num, &prev_row_encoded);
                //             } 

                //             // if we have a push me, it's now over and we need to push it
                //             if push_me_from_above > 0 && [0, -1].contains(&prev_row_encoded[i]){
                //                 add_nums.push(push_me_from_above);
                //                 push_me_from_above = 0;
                //             } 
                //             println!("{:?}", &push_me_from_above);

    
                //             if i < prev_row_encoded.len() - 1 && prev_row_encoded[i+1] > 0 {
                //                 // if we still have a push me from prev check, it is the same num and we can push it
                //                 if push_me_from_above > 0 {
                //                     add_nums.push(push_me_from_above);
                //                 } else {
                //                     add_nums.push(prev_row_encoded[i+1]);
                //                 }
                //                 println!("{:?}", &push_me_from_above);
                //                 println!("{:?}", &prev_row_encoded[i+1]);

                                
                //                 // if we still have a push me from prev check, it is the same num and we can push it
                //                 // add_nums.push(prev_row_encoded[i+1]);
    
                //                 // remove all instances of the num from prev_row_encoded to prevent double addition
                //                 // let repl_num = &prev_row_encoded[i+1];
                //                 // prev_row_encoded = repl_num_w_zero(repl_num, &prev_row_encoded);
                //             } else {
                //                 add_nums.push(push_me_from_above);
                //             }
                //         }
                        
                //         // this row only check adjacent, no need for a push me
                //         if i > 0 && encoded_row[i-1] > 0 {
                //             add_nums.push(encoded_row[i-1]);
                //             println!("{:?}", &encoded_row[i-1]);

                //             // remove all instances of the num from prev_row_encoded to prevent double addition
                //             // let repl_num = &encoded_row[i-1];
                //             // encoded_row = repl_num_w_zero(repl_num, &encoded_row);
                //         }

                //         if i < encoded_row.len() - 1 && encoded_row[i+1] > 0 {
                //             add_nums.push(encoded_row[i+1]);
                //             println!("{:?}", &encoded_row[i+1]);

                //             // remove all instances of the num from prev_row_encoded to prevent double addition
                //             // let repl_num = encoded_row[i-1];
                //             // encoded_row = repl_num_w_zero(&repl_num, &encoded_row);
                //         }
                //     }
                // }

                // // then compare prev row symbols 
                // if prev_row_encoded.len() > 0 {
                //     let mut push_me_from_below = 0;

                //     for i in 0..prev_row_encoded.len() - 1 { 
                //     // it is a symbol, check the nums in the previous row and this row
                //         if prev_row_encoded[i] == -1 {
                //             if encoded_row[i] > 0 {
                //                 push_me_from_below = encoded_row[i-1];
                //                 // add_nums.push(encoded_row[i]);
                //                 println!("{:?}", &push_me_from_below);

                //                 // remove all instances of the num from prev_row_encoded to prevent double addition
                //                 // let repl_num = &prev_row_encoded[i];
                //                 // prev_row_encoded = repl_num_w_zero(repl_num, &prev_row_encoded);
                //             }

                //             if i > 0 && encoded_row[i-1] > 0 {
                //                 if push_me_from_below == 0 {
                //                     push_me_from_below = encoded_row[i];
                //                     println!("{:?}", &push_me_from_below);
                //                 }
                //                 // add_nums.push(encoded_row[i-1]);

                //                 // remove all instances of the num from prev_row_encoded to prevent double addition
                //                 // let repl_num = &prev_row_encoded[i-1];
                //                 // prev_row_encoded = repl_num_w_zero(repl_num, &prev_row_encoded);
                //             } 

                //             // if we have a push me, it's now over since this is a 0 or -1, and we need to push it
                //             if push_me_from_below > 0 && [0, -1].contains(&encoded_row[i]){
                //                 add_nums.push(push_me_from_below);
                //                 push_me_from_below = 0;
                //             }
                //             println!("{:?}", &push_me_from_below);

                //             if i < encoded_row.len() - 1 && encoded_row[i+1] > 0 {
                //                 println!("{:?}", &push_me_from_below);

                //                 if push_me_from_below > 0 {
                //                     add_nums.push(push_me_from_below);
                //                 } else {
                //                     add_nums.push(encoded_row[i+1]);
                //                 }
                //                 // add_nums.push(encoded_row[i+1]);

                //                 // remove all instances of the num from prev_row_encoded to prevent double addition
                //                 // let repl_num = &prev_row_encoded[i+1];
                //                 // prev_row_encoded = repl_num_w_zero(repl_num, &prev_row_encoded);
                //             } else {
                //                 add_nums.push(push_me_from_below);
                //             }
                //         }
                //     }
                // }

                // println!("{:?}", &add_nums);

                


                // // replace w/ new row
                // prev_row_encoded = encoded_row;

                // // sum the add_nums to final_answer
                // for n in add_nums{
                //     final_answer += n;
                // }
            }
        
        }
        encoded_table.push(zero_row.clone());
        symbol_table.push(zero_row.clone());

        println!("{:?}", &encoded_table);
        println!("{:?}", &symbol_table);

        // now we go through the values table and expand the 1s to the 8 adjacent points in the 3 relevant rows: top, this, bottom
        // and add in where there's a 1
        // use a new table for it for easier debugging
        let mut processed_table:Vec<Vec<i32>> = Vec::new();

        for (y, row) in encoded_table.iter().enumerate() {
            for (x, val) in row.iter().enumerate() {
                if symbol_table[y][x] == 1{
                    // use the a window of sets
                    let top_row = HashSet::from([encoded_table[y-1][x-1]*symbol_table[y][x], encoded_table[y-1][x]*symbol_table[y][x], encoded_table[y-1][x+1]*symbol_table[y][x]]);
                    let mid_row = HashSet::from([encoded_table[y][x-1]*symbol_table[y][x], encoded_table[y][x]*symbol_table[y][x], encoded_table[y][x+1]*symbol_table[y][x]]);
                    let bottom_row = HashSet::from([encoded_table[y+1][x-1]*symbol_table[y][x], encoded_table[y+1][x]*symbol_table[y][x], encoded_table[y+1][x+1]*symbol_table[y][x]]);

                    println!("{:?}", &top_row);
                    println!("{:?}", &mid_row);
                    println!("{:?}", &bottom_row);
                    println!("");

                    for n in top_row{
                        final_answer += n;
                    }

                    for n in mid_row{
                        final_answer += n;
                    }

                    for n in bottom_row{
                        final_answer += n;
                    }
                }
            }
        }
        

    }

    println!("{}", final_answer);
}

fn repl_num_w_zero(repl_num:&i32, vec:&Vec<i32>) -> Vec<i32> {
    let mut encoded_row:Vec<i32> = Vec::new();

    for num in vec.iter() {
        if num == repl_num{
            encoded_row.push(0);
        } else {
            encoded_row.push(*num);
        }
    }
    return encoded_row;
}
// read each line, read each char to see if it is a digit...
// if it is, store initial index
// keep reading until either end of line or we reach a non-digit
// store the end index
// parse the int between start and end
// append the parsed int for each index, in the row
// if it isn't a digit, append 0 for "." or "-1" for any symbol

// row_num: [val at idx for row]
// 467..114..
// v: [
//     467,
//     467,
//     467,
//     0,
//     0,
//     114,
//     114,
//     114,
//     0,
//     0,
// ]
// ...*......
// v: [
//     0,
//     0,
//     0,
//     -1,  any symbol
//     0,
//     0,
//     0,
//     0,
//     0,
//     0,
// ]
// once the row has been encoded, get the indices of any -1s
// for each -1 check the prev row and this row for any positive integer value within (idx-1, idx+1), accounting for start/end of rows

// from https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html  
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
