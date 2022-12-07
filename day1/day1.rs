use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    let max = handle_input_part_2("input1.txt");
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

fn handle_input_part_1(filename: &str) -> i32 {
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

fn handle_input_part_2(filename: &str) -> i32 {
    if let Ok(lines) = read_lines(filename) {
        // keep track of top 3
        // something for sure goes in top 3 if it's larger than smallest of current top 3
        let mut top3: top3_t = {
            all_set: false,
            t3_items: HashSet::new(),
            smallest: f32::INFINITY,
        };

        let mut curr_group_sum: i32 = 0;

        for line in lines {
            if let Ok(ip) = line {
                if ip == "" {
                    insert_into_top3(top3, curr_group_sum);
                    curr_group_sum = 0;
                } else {
                    let line_as_int: i32 = ip.parse().unwrap();
                    curr_group_sum += line_as_int;
                }
            }
        }

        // return sum of top 3
        let mut sum = 0;
        for item in top3.t3_items {
            sum += item;
        }

        return sum;
    } else {
        panic!();
    }
}

fn insert_into_top3(t3_p: top3_t, val: i32) {
    if !t3_p.all_set {
        t3_p.t3_items.insert(val);
        set_smallest(t3_p);
    } else {
        if val > t3_p.smallest {
            t3_p.t3_items.remove(t3_p.smallest);
            set_smallest(t3_p);
        }
    }
}

// desired traits for data structure:
//  - easy to check minimum element in top 3
//  - easy to place new element in top 3 if it belongs, replacing minimum
struct top3_t {
    all_set: bool,
    t3_items: HashSet<i32>,
    smallest: i32,
}

fn set_smallest(t3: t3_p) {
    let mut smallest = f32::INFINITY;
    
    for item in t3.t3_items {
        if item < smallest {
            smallest = item;
        }
    }

    t3.smallest = smallest;
}
