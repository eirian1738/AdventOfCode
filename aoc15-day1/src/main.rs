use std::fs;

fn find_floor(s: &str) -> i32 {
    let mut actual_floor = 0;
    
    // chars transformation for looping
    for i in s.chars() {
        if i == '(' {
            actual_floor += 1;
        }
        else if i == ')' {
            actual_floor -= 1;
        }
    }
    actual_floor
}


// for the second part a separate solution is chosen 
fn find_basement_position(s: &str) -> Option<usize> {
    let mut actual_floor = 0;

    for (i,v) in s.chars().enumerate() {
        if v == '(' {
            actual_floor += 1;
        }
        else if v == ')' {
            actual_floor -= 1;
        }
        if actual_floor == -1 {
            // +1 is needed due to 0-indexing in rust
            return Some(i+1);
        }
    }
    None
}


fn main() {

    let puzzle_input = fs::read_to_string("puzzle_input.txt")
        .expect("FAILED TO READ FILE");
    
    let solution_part_one = find_floor(&puzzle_input);
    println!("Solution first part: {}", solution_part_one);


    let solution_part_two = find_basement_position(&puzzle_input).unwrap();
    println!("Solution second part: {}", solution_part_two);

}
