pub mod utils;
use std::{env, usize};

// TODO: create a function to separate the Brainfuck interpreter from the main program
// TODO: unit tests(?)
fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!(
            "Problem parsing arguments: one file argument should be provided, received {}",
            args.len() - 1
        );
        std::process::exit(1);
    }
    let content = utils::read_file(&args[1])?;
    const SIZE: usize = 30000;
    let mut arr: [u8; SIZE] = [0; SIZE];
    let mut position: usize = 0;
    let mut loop_beginnings: Vec<usize> = Vec::new();
    let mut i: usize = 0;
    while i < content.len() {
        let instruction = content.as_bytes()[i] as char;
        match instruction {
            '+' => arr[position] = u8::wrapping_add(arr[position], 1u8),
            '-' => arr[position] = u8::wrapping_sub(arr[position], 1u8),
            '>' => position += 1,
            '<' => position -= 1,
            ',' => (), // TODO: implement input element;
            '.' => print!("{}", char::from(arr[position])),
            '[' => loop_beginnings.push(i),
            ']' => {
                if arr[position] == 0 {
                    loop_beginnings.pop();
                } else {
                    match loop_beginnings.last() {
                        Some(value) => i = *value,
                        None => (),
                    }
                }
            }
            _ => (),
        }
        i += 1;
    }
    println!();
    Ok(())
}
