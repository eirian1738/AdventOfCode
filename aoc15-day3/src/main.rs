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

fn main() {

    let puzzle_input = read_input("puzzle_input.txt").unwrap();

    let mut actual_pos = Coords {
        x: 0,
        y: 0,
    };
    
    // vector of consecutive positions on the grid
    let mut positions: Vec<Coords> = vec![];

    positions.push(actual_pos.clone());
    
    for i in puzzle_input.chars() {

        // matching the possible movement chars 
        match i {
            '^' => actual_pos.move_up(),
            'v' => actual_pos.move_down(),
            '<' => actual_pos.move_left(),
            '>' => actual_pos.move_right(),
            _ => (),
        }
        positions.push(actual_pos.clone());
    }

    // the first sorting sorts all elements by the x coordinate
    positions
        .sort_by( |a, b| (a.x).partial_cmp(&(b.x)).unwrap());

    // the second sorting sorts all elements by the y coordinate
    positions
        .sort_by( |a, b| (a.y).partial_cmp(&(b.y)).unwrap());


    println!("Positions list length {}", puzzle_input.len());
    
    // eliminate all duplicates in a sorted array
    positions.dedup();
            
    println!("Solution first part: {}", positions.len());
}
