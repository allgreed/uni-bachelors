use std::io::prelude::*;
use std::fs::File;


const INPUT_FILENAME: &str = "dane";
const OUTPUT_FILENAME: &str = "wyniki";


fn main() {
    loop {
        let mut raw_input = String::new();
        let mut input_file = match File::open(INPUT_FILENAME) {
            Ok(x) => x,
            Err(_) => { continue; },
        };
        match input_file.read_to_string(&mut raw_input) {
            Ok(_) => {},
            Err(_) => { continue; },
        }
        if raw_input == "" {
            continue;
        }
        File::create(INPUT_FILENAME)
            .unwrap();

        let input: i32 = raw_input.trim().parse()
            .expect("client input is an integer");

        let result = compute_small_polynomial(input);

        let mut output_file = File::create(OUTPUT_FILENAME)
            .unwrap();
        output_file.write_all(result.to_string().as_bytes())
            .unwrap();
    }
}

fn compute_small_polynomial(x: i32) -> i32
// 5x^3 + 4x^2 + 3x + 2
{
    2 + x*(
        3 + x*(
            4 + x*(
                5
            )
        )
    )
}
