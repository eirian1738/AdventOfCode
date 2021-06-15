use std::fs;
use std::io;

fn read_input(name: &str) -> Result<String, io::Error> {
    let file = fs::read_to_string(name);
    
    let file = match file {
        Ok(file) => Ok(file),
        Err(e) => Err(e),
    };

    file
}

// Grid-based solution
// working with a 2-based dimensional grid of coordinates every momevent means a new coordinate
// saving all coordinates in a vec and eliminating duplicates give the number of houses the receive
// at least one present

#[derive(Debug,Clone)]
#[derive(PartialEq,Eq)]
struct Coords {
    x: i32,
    y: i32,
}

// movement functions
impl Coords {
    fn move_up(&mut self) {
        self.y += 1;
    }
    fn move_down(&mut self) {
        self.y -= 1;
    }
    fn move_right(&mut self) {
        self.x += 1;
    }
    fn move_left(&mut self) {
        self.x -= 1;
    }
}


// This function parse the movements of the given input string with a match based on the 4 possible
// symbols
fn parse_movements(s: &str) -> Vec<Coords> {
    let mut actual_pos = Coords {
        x: 0,
        y: 0,
    };

    let mut positions: Vec<Coords> = vec![];

    for i in s.chars() {
        match i {
            '^' => actual_pos.move_up(),
            'v' => actual_pos.move_down(),
            '<' => actual_pos.move_left(),
            '>' => actual_pos.move_right(),
            _ => (),
        }
        positions.push(actual_pos.clone());
    }
    
    positions
}

// This function sort a vector of coordinates, removes duplicates and return the size of the final
// vec


fn total_houses(mut p: Vec<Coords>) -> usize {
    p.sort_by(|a,b| (a.x).partial_cmp(&(b.x)).unwrap());
    p.sort_by(|a,b| (a.y).partial_cmp(&(b.y)).unwrap());
    p.dedup();
 
    p.len()
}


// This function breaks the input string in 2 substrings
// Santa movements are in even position in the puzzle_input
// Robo-Santa movements are in odd position in the puzzle_input
fn break_input(s: &str) -> (String,String) {
    let s1 = s
        .chars()
        .enumerate()
        .filter(| (i,_) | i % 2 == 0)
        .map(|(_,e)| e)
        .collect::<String>();

    let s2 = s
        .chars()
        .enumerate()
        .filter(| (i,_) | i % 2 != 0)
        .map(|(_,e)| e)
        .collect::<String>();

    (s1,s2)
}

fn main() {

    let puzzle_input = read_input("puzzle_input.txt").unwrap();

    // first part solution
    
    let coords = parse_movements(&puzzle_input);
    let first_sol = total_houses(coords);

    println!("Solution first part: {}", first_sol);

    // second part solution
    
    // break total movements in parts (between Santa and Robo-Santa)
    let (santa_input,robo_input) = break_input(&puzzle_input);

    // parse movements of the 2 inputs separately
    let mut santa_moves = parse_movements(&santa_input);
    let robo_moves = parse_movements(&robo_input);

    // Merge the coordinates togheter for finding duplicates
    santa_moves.extend(robo_moves);

    // find duplicates
    let second_sol = total_houses(santa_moves);

    println!("Solution second part: {}", second_sol);

}
