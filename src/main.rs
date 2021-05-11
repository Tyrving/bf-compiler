use std::io::stdin;

const TAPE_LEN:usize = 256;

fn simple_readline() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Rip");
    input
}

fn print_tape(tape :[u8; TAPE_LEN], pointer :usize) {
    let mut last_nonzero :usize = 0;
    let max_print :usize;

    for (i, _item) in tape.iter().enumerate(){
        if tape[i] != 0 {
            last_nonzero = i;
        }
    }
    if pointer>last_nonzero {
        max_print = pointer+2;
    } else {
        max_print = last_nonzero+2;
    }
    for i in 0..max_print+1 {
        if pointer == i {
            print!("p=>");
        }
        print!("{}|", tape[i]);
    }
    println!();
}

fn main() {
    println!("Welcome to Mark's BF compiler, written in rust!");
    println!("Input BF program below. I do not support nested loops.");
    let mut max_pointer:usize = 0;
    let mut pointer: usize = 0; //use for tape index
    let mut input_index: usize = 0; //current character to check
    let mut tape: [u8; TAPE_LEN] = [0; TAPE_LEN]; //init tape to 0's
    let input: String = String::from(simple_readline()); //input characters
    let mut skip_forward: bool = false; //skip over loop
    let mut skip_back: bool = false;
    let mut return_string: String = String::new(); //add output characters on '.' to this variable


    while input_index < input.len() {

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
                tape[pointer] = tape[pointer].wrapping_add(1);
                input_index += 1;
            }
            '-' => {
                tape[pointer] = tape[pointer].wrapping_sub(1);
                input_index += 1;
            }
            '>' => {
                pointer += 1;
                input_index += 1;
                if pointer > max_pointer {
                    max_pointer = pointer;
                }
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
