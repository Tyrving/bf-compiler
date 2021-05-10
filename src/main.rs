use std::io::stdin;
use std::env::args;
const TAPE_LEN:usize = 256;

fn print_tape(tape :[u8; TAPE_LEN], pointer :usize) -> [u8; TAPE_LEN] {
    let mut last_nonzero :usize;

    for i in 0..TAPE_LEN-1 {
        if tape[i] != 0 {
            last_nonzero = index;
        }
    }

    for byte in tape.iter(){
        print!("{}|", byte);
    }

    println!();

}

fn main() {
    println!("Welcome to Mark's BF compiler, written in rust!");
    println!("Input BF program below. I do not support nested loops.");

    let mut pointer: usize = 0; //use for tape index
    let mut input_index: usize = 0; //current character to check
    let mut tape: [u8; TAPE_LEN] = [0; TAPE_LEN]; //init tape to 0's
    let mut input: String = String::new(); //input characters
    let mut skip_forward: bool = false; //skip over loop
    let mut skip_back: bool = false;
    let mut return_string: String = String::new(); //add output characters on '.' to this variable

    stdin() //  read line of input characters
        .read_line(&mut input)
        .expect("Failed to read line");


    while input_index < input.len()-1 {

        //assign current character to variable
        let current_char: char = input.as_bytes()[input_index] as char;

        if skip_back {
            if current_char == '[' {
                skip_back = false;
                continue;
            }
            input_index -= 1;
            continue;
        } else if skip_forward {
            if current_char == ']' {
                skip_forward = false;
                continue;
            }
            input_index += 1;
            continue;
        }

        //print out current tape
        print_tape(tape, pointer);

        match current_char {
            '+' => {
                tape[pointer] += 1;
                input_index += 1;
            }
            '-' => {
                tape[pointer] -= 1;
                input_index += 1;
            }
            '>' => {
                pointer += 1;
                input_index += 1;
            }
            '<' => {
                pointer -= 1;
                input_index += 1;
            }
            ',' => {

            }
            '.' => {
                return_string += &(tape[pointer] as char).to_string();
                input_index += 1;
            }
            '[' => {
                if tape[pointer] == 0 {
                    skip_forward = true;
                } else {
                    input_index += 1;
                }
            }
            ']' => {
                if tape[pointer] == 0 {
                    input_index += 1;
                } else {
                    skip_back = true;
                }
            }
            '\n' => break, //end on newline
            _ => (), //ignore other characters
        }
        println!(
            "Current Char: {} ({}) of {}. Value {}",
            input_index,
            current_char,
            input.len(),
            tape[pointer]
        );
    }
    println!(); //newline and print outputs
    println!("{}", return_string);
}
