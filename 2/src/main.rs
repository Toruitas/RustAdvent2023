use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    let mut final_answer:u32 = 0;

    // total: 12 red cubes, 13 green cubes, and 14 blue cubes
    let max_cubes: HashMap<&str, u32> = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14)
    ]);

    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(game) = line {
                final_answer += playable_game(game, &max_cubes);
            }
        }
    }

    println!("{}", final_answer);
}



// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
fn playable_game(game: String, max_cubes: &HashMap<&str, u32>) -> u32{
    // split on ":" and the 1st substring has the game ID
    let game_id_and_games = game.trim().split(":").collect::<Vec<&str>>();
    println!("{:?}", game_id_and_games);

    // split substring 1 on " " and the 2nd value is the game ID
    let id_str = game_id_and_games[0].trim().split(" ").collect::<Vec<&str>>();
    println!("{:?}", id_str);

    // let id =  id_str[1].parse::<u32>().unwrap();
    let id:u32 = match id_str[1].trim().parse(){
        Ok(num) => num,
        Err(e) => panic!("Can't convert to u32: {e}"),
    };
    println!("{:?}", id);

    // split substring 2 by ";" to get the 'hand'
    let hands = game_id_and_games[1].trim().split(";").collect::<Vec<&str>>();
    println!("{:?}", hands);
    
    // for each, split by "," to get number and color
    for hand in hands {
        let hand_colors = hand.trim().split(",").collect::<Vec<&str>>();

        for color in hand_colors{
            // split by space " " and idx 0 is the num while 1 is the color
            let color_and_count = color.trim().split(" ").collect::<Vec<&str>>();
            // if color count is greater than the max for that color, return 0
            println!("{:?}", color_and_count);
            let count_int: u32 = match color_and_count[0].trim().trim().parse(){
                Ok(num) => num,
                Err(e) => panic!("Can't convert to u32: {e}"),
            };
            println!("{:?}", count_int);

            println!("{:?}", max_cubes.get(color_and_count[1]).cloned().unwrap_or(0));

            if max_cubes.get(color_and_count[1]).cloned().unwrap_or(0) < count_int{
                println!("{:?}", 0);
                return 0;
            }
        }
    };

    return id;
}


// from https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html  
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
