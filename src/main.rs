use std::io;

fn main() {
    println!("Welcome to Mark's BF compiler, written in rust!");
    let mut pointer:usize = 0;
    let mut in_char_num: usize = 0;
    let mut tape: [u8; 10] = [0; 10];
    let mut input: String = String::new();
    let mut skip_forward: bool = false;
    let mut skip_back: bool = false;

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    while in_char_num < input.len() {
        let char_char: char = input.as_bytes()[in_char_num] as char;

        for byte in tape.iter() {
            print!("{}|", byte);
        }

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
            '.' => {
                print!("{}", tape[pointer] as char);
                in_char_num += 1;
            }
            '[' => {
                if tape[pointer] == 0 {
                    skip_forward = true;
           //         pointer+=1;
                } else {
                    in_char_num += 1;
                }
            }
            ']' => {
                if tape[pointer] == 0 {
                    in_char_num += 1;
                } else {
                    skip_back = true;
                //    pointer-=1;
                }
            }
            '\n' => break,
            _ => println!("Whoopsie")
        }
        println!(
            "Current Char: {}--{} of {}. Value {}",
            in_char_num,
            char_char,
            input.len(),
            tape[pointer]
        );
    }
}
