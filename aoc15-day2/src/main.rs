use std::fs;

// this struct contains paper's geometric information
#[derive(Debug)]
struct Paper {
    l: u32,
    w: u32,
    h: u32,
}

impl Paper {
    fn volume(&self) -> u32 {
        self.l * self.w * self.h
    }
    fn calc_main_area(&self) -> u32 {
        let area = 2 * self.l * self.w + 2 * self.w * self.h + 2* self.h * self.l;
        area
    }
}



// read file function
fn read_file(name: &str) -> String {
    let file = fs::read_to_string(name);

    let file = match file {
        Ok(f) => f,
        Err(e) => panic!("{}", e),
    };

    file
}

////// Functions require for part one
// this function is created according to the file format of puzzle_input
fn parse_string(s: &str) -> Paper {
    let v: Vec<&str> = s
        .split("x")
        .collect();

    let p = Paper {
        l: v[0].parse().unwrap(),
        w: v[1].parse().unwrap(),
        h: v[2].parse().unwrap(),
    };
    p
}

// this function parses all strings line by line and save them in a vector of papers
fn parse_all_strings(s: &str) -> Vec<Paper> {
    let mut v: Vec<Paper> = vec![];
    
    for line in s.lines() {
        v.push(parse_string(line));
    }

    v
}


// find the minimum product to add to the total area of paper
fn min_prod(p: &Paper) -> u32 {
    let prod1 = p.l * p.w;
    let prod2 = p.l * p.h;
    let prod3 = p.w * p.h;

    let mut min = prod1;
    if prod2 < min {
        min = prod2;
    }
    if prod3 < min {
        min = prod3;
    }
    min
}

/////// End of functions require for part one


/////// Functions required for parto two
fn smallest_perimeter(p: &Paper) -> u32 {
    let mut v = vec![p.l,p.w,p.h];
    v.sort();

    2 * v[0] + 2 * v[1]
}


fn main() {
    
    // This is the first part solution
    let puzzle_input = read_file("puzzle_input.txt");
    let papers = parse_all_strings(&puzzle_input);
    let mut total_area = 0;

    for p in &papers {
        total_area += p.calc_main_area();
        total_area += min_prod(p);
    }

    println!("First part solution {}", total_area);
    // end of the first part solution
    
    // Second part solution
    
    let mut ribbon = 0;
    for p in &papers {
        ribbon += smallest_perimeter(p);
        ribbon += p.volume();
    }

    println!("Second part solution: {}", ribbon);

}

