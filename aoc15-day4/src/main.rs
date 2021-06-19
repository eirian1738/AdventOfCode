extern crate md5;


// This funciton takes as input the puzzle_input and a slice with the number of zeros 
fn find_num(s: &str, n_zeros: &str) -> u32 {
    
    let mut result = 0;
    let len = n_zeros.len();

    let mut i = 0;
    
    loop {

        // Concatenate the puzzle input with the numbers
        let mut puzzle_input = String::from(s);
        let s1 = String::from(i.to_string());
        puzzle_input.push_str(&s1);

        // Evaluation of the digest MD5
        let digest = md5::compute(puzzle_input.as_bytes());
        let d = format!("{:x}", digest);

        // Compare the first len(n_zeros) characters - Break if if find one (is the minimum
        // possible)
        if &d[0..len] == n_zeros {
            result = i;
            break;
        }
        i += 1;
    }

    result
}



fn main() {

    let s = "yzbqklnj";
    
    // Solution first part
    let n_zeros_first = "00000";

    let result_first = find_num(s,n_zeros_first);
    println!("First part solution {}", result_first);

    // Solution second part
    let n_zeros_second = "000000";
    let result_second = find_num(s,n_zeros_second);
    println!("Second part solution {}", result_second);
    
}
