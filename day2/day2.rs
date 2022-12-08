use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    let res = handle_input_part_1("input2.txt");
    println!("{res}")
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn handle_input_part_1(filename: &str) -> i32 {
    if let Ok(lines) = read_lines(filename) {
        let mut total_score = 0;

        for line in lines {
            if let Ok(ip) = line {
                let line_score = process_line(ip);
                total_score += line_score;
            }
        }

        return total_score;
    } else {
        panic!();
    }
}

fn parse_line(line: String) -> (&str) {
    let mut split_line = line.split(' ');
    let opponent_play = split_line.nth(0).unwrap();
    let our_play = split_line.nth(0).unwrap(); // must use nth(0) again since iterator consumes values in line above
    
    return opponent_play, our_play;
}

fn process_line(line: String) -> i32 {
    // split into opponent's choice and our choice
    

    let score_from_choice = score_choice(our_play);
    let score_from_outcome = score_game(opponent_play, our_play);

    return score_from_choice + score_from_outcome;
}

fn score_choice(choice: &str) -> i32 {
    match choice {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => todo!(),
    }
}

fn score_game(opp_play: &str, our_play: &str) -> i32 {
    match (opp_play, our_play) {
        ("A", "X") => 3,
        ("A", "Y") => 6,
        ("A", "Z") => 0,
        ("B", "X") => 0,
        ("B", "Y") => 3,
        ("B", "Z") => 6,
        ("C", "X") => 6,
        ("C", "Y") => 0,
        ("C", "Z") => 3,
        _ => todo!(),
    }
}

fn handle_input_part_2()
