use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    let max = handle_input("input1.txt");
    println!("{max}")
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

fn handle_input(filename: &str) -> i32 {
    if let Ok(lines) = read_lines(filename) {
        let mut curr_group_sum = 0;
        let mut max_group_sum = 0;

        for line in lines {
            if let Ok(ip) = line {
                if ip == "" {
                    if curr_group_sum > max_group_sum {
                        max_group_sum = curr_group_sum
                    }

                    curr_group_sum = 0;
                } else {
                    let line_as_int: i32 = ip.parse().unwrap();
                    curr_group_sum += line_as_int;
                }
            }
        }

        return max_group_sum;
    } else {
        panic!();
    }
}
