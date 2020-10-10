#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use std::io::prelude::*;
use std::io::{self, Write};
use std::{process, env};
use std::fmt::Display;
use std::fs::File;


mod caesar;
mod vigenere;


const PLAINTEXT_FILENAME:&str = "plain.txt";
const CIPHER_FILENAME:&str = "crypto.txt";
const DECRYPTED_FILENAME:&str = "decrypt.txt";
const KEY_FILENAME:&str = "key.txt";
const KEY_RECOVERED_FILENAME:&str = "key-crypto.txt";
const UNPROCESSED_FILENAME:&str = "orig.txt";


#[derive(Debug)]
enum Action {
    Encrypt, Decrypt, Sign, Verify, Keygen
}


fn main() {
    let argv: Vec<String> = env::args().collect();
    let args = &argv[1 .. ];
    
    let action = parse_arguemnts(args)
        .unwrap_or_else(|e| exit_err(e, "parsing arguments"));

    match action {
        Action::Encrypt => {
            let plaintext = read_from(PLAINTEXT_FILENAME).trim().to_string();
            let key = vigenere::Key::parse(&read_from(KEY_FILENAME))
                .unwrap_or_else(|e| exit_err(e, "parsing key"));

            write_to(CIPHER_FILENAME, vigenere::encrypt(&plaintext, &key) + "\n");
        }
        Action::Decrypt => {
            let ciphertext = read_from(CIPHER_FILENAME).trim().to_string();
            let key = vigenere::Key::parse(&read_from(KEY_FILENAME))
                .unwrap_or_else(|e| exit_err(e, "parsing key"));

            write_to(DECRYPTED_FILENAME, vigenere::decrypt(&ciphertext, &key) + "\n");
        }
        Action::Prepare => {
            let plain = read_from(UNPROCESSED_FILENAME)
                .trim()
                .chars()
                .filter(|c| c.is_ascii_alphabetic())
                .collect::<String>()
                .to_lowercase();

            write_to(PLAINTEXT_FILENAME, plain);
        }
        Action::Crack => {
            let ciphertext = read_from(CIPHER_FILENAME).trim().to_string();

            let key = vigenere::crack(&ciphertext);

            write_to(DECRYPTED_FILENAME, vigenere::decrypt(&ciphertext, &key) + "\n");
            write_to(KEY_RECOVERED_FILENAME, format!("{}", &key) + "\n");
        }
    };
}


fn parse_arguemnts(args: &[String]) -> Result<Action, &'static str>
{
    if args.len() != 1 {
        return Err("wrong number of arguments - exactly 1 arguments was expected");
    }

    flag_to_action(&args[0])
}


fn flag_to_action(flag: &str) -> Result<Action, &'static str> {
    match flag {
        "-e" => Ok(Action::Encrypt),
        "-d" => Ok(Action::Decrypt),
        "-p" => Ok(Action::Prepare),
        "-k" => Ok(Action::Crack),
        _ => Err("unknown action, try *one* of '-edpk' instead"),
    }
}


fn read_from(filename: &str) -> String
{
    let mut buffer = String::new();
    let mut file = File::open(filename).unwrap();
    file.read_to_string(&mut buffer).unwrap();
    buffer
}

fn write_to(filename: &str, content: String) -> ()
{
    let mut file = File::create(filename)
        .unwrap();
    file.write_all(content.as_bytes())
        .unwrap();
}

fn exit_err<T: Display>(message: T, context: T) -> !
{
    let _ = writeln!(&mut io::stderr(), "ERROR at {}: {}", context, message);
    process::exit(1);
}
