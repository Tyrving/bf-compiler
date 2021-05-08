use std::io;

fn main() {
    println!("Welcome to Mark's BF compiler, written in rust!");
    let mut pointer: usize = 0; //use for tape index
    let mut in_char_num: usize = 0; //current character to check
    let mut loop_depth: usize = 0; //for loops within loops
    let mut tape: [u8; 25] = [0; 25]; //init tape to 0's
    let mut input: String = String::new(); //input characters
    let mut skip_forward: bool = false; //skip over loop
    let mut skip_back: bool = false;
    let mut return_string: String = String::new(); //add output characters on '.' to this variable

    io::stdin() //  read line of input characters
        .read_line(&mut input)
        .expect("Failed to read line");

    //use this to go over each character,
    // incrementing and decrementing in_char_num for loops and such
    while in_char_num < input.len()-1 {

        //assign current character to variable
        let char_char: char = input.as_bytes()[in_char_num] as char;

        if skip_back {
            if char_char == '[' {
                skip_back = false;
                continue;
            }
            in_char_num -= 1;
            continue;
        } else if skip_forward {
            if char_char == ']' {
                skip_forward = false;
                continue;
            }
            in_char_num += 1;
            continue;
        }

        //print out current tape
        for byte in tape.iter() {
            print!("{}|", byte);
        }
        println!();

        match char_char {
            '+' => {
                tape[pointer] += 1;
                in_char_num += 1;
            }
            '-' => {
                tape[pointer] -= 1;
                in_char_num += 1;
            }
            '>' => {
                pointer += 1;
                in_char_num += 1;
            }
            '<' => {
                pointer -= 1;
                in_char_num += 1;
            }
            ',' => {

            }
            '.' => {
                return_string += &(tape[pointer] as char).to_string();
                in_char_num += 1;
            }
            '[' => {
                if tape[pointer] == 0 {
                    skip_forward = true;
                } else {
                    in_char_num += 1;
                    loop_depth += 1;
                }
            }
            ']' => {
                if tape[pointer] == 0 {
                    in_char_num += 1;
                    loop_depth -= 1;
                } else {
                    skip_back = true;
                }
            }
            '\n' => break, //end on newline
            _ => (), //ignore other characters
        }
        println!(
            "Current Char: {} ({}) of {}. Value {}",
            in_char_num,
            char_char,
            input.len(),
            tape[pointer]
        );
    }
    println!(); //newline and print outputs
    println!("{}", return_string);
}
