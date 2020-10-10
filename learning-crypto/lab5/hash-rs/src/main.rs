use std::fs::File;
use std::io::Read;


const BEFORE_FILENAME:&str = "hash.txt";
const AFTER_FILENAME:&str = "hash_.txt";

fn main() {
    let _a = read_from(BEFORE_FILENAME);
    let _b = read_from(AFTER_FILENAME);
    let (a, b) = (_a.trim(), _b.trim());

    for (la, lb) in a.split("\n").zip(b.split("\n"))
    {
        let mut hamming_weight = 0;

        for (ca, cb) in la.chars().zip(lb.chars())
        {
            let va = u8::from_str_radix(&ca.to_string(), 16).unwrap();
            let vb = u8::from_str_radix(&cb.to_string(), 16).unwrap();
            
            let mut x: u8 = va ^ vb;
            while x != 0
            {
                hamming_weight += 1;
                x &= x - 1;
            }
        }

        let total_bits = la.len() * 4;
        println!("{:>3} / {:>3} = {:>5.2}%", hamming_weight, total_bits, hamming_weight as f64 / total_bits as f64 * 100.0);
    }
}


fn read_from(filename: &str) -> String
{
    let mut buffer = String::new();
    let mut file = File::open(filename).unwrap();
    file.read_to_string(&mut buffer).unwrap();
    buffer
}
