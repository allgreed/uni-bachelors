use std::io;
use std::io::prelude::*;
use std::fs::File;


const INPUT_FILENAME: &str = "dane";
const OUTPUT_FILENAME: &str = "wyniki";


fn main() {
    let mut raw_input = String::new();
    io::stdin().read_line(&mut raw_input)
        .unwrap();

    let input: i32 = raw_input.trim().parse()
        .expect("keyboard input is an integer");

    let mut input_file = File::create(INPUT_FILENAME)
        .unwrap();
    input_file.write_all(input.to_string().as_bytes())
        .unwrap();

    loop {
        let mut raw_output = String::new();
        let mut output_file = match File::open(OUTPUT_FILENAME) {
            Ok(x) => x,
            Err(_) => { continue; },
        };
        match output_file.read_to_string(&mut raw_output) {
            Ok(_) => {},
            Err(_) => { continue; },
        }
        if raw_output == "" {
            continue;
        }
        File::create(OUTPUT_FILENAME)
            .unwrap();

        let output: i32 = raw_output.trim().parse()
            .expect("server output is an integer");

        println!("Result: {}", output);

        break;
    }
}
