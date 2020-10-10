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


mod xor;


const EXPECTED_KEY_LENGTH: usize = 32;

const PLAINTEXT_FILENAME:&str = "plain.txt";
const CIPHER_FILENAME:&str = "crypto.txt";
const DECRYPTED_FILENAME:&str = "decrypt.txt";
const KEY_FILENAME:&str = "key.txt";
const UNPROCESSED_FILENAME:&str = "orig.txt";


#[derive(Debug)]
enum Action {
    Encrypt, Crack, Prepare
}


fn main() {
    let argv: Vec<String> = env::args().collect();
    let args = &argv[1 .. ];
    
    let action = parse_arguemnts(args)
        .unwrap_or_else(|e| exit_err(e, "parsing arguments"));

    match action {
        Action::Encrypt => {
            let plaintext = read_from(PLAINTEXT_FILENAME)
                .chars()
                    .filter(|c| c != &'\n')
                .collect::<String>()
                ;

            // TODO: validate that key.len() matches KEY_LENGTH
            let key = xor::Key::parse(&read_from(KEY_FILENAME).trim())
                .unwrap_or_else(|e| exit_err(e, "parsing key"));

            let encrypted = xor::encrypt(&plaintext, &key)
                .chars()
                .collect::<Vec<char>>()
                .chunks(EXPECTED_KEY_LENGTH)
                .map(|c| c.iter().collect::<String>())
                .collect::<Vec<String>>()
                .join("\n")
                ;

            write_to(CIPHER_FILENAME, encrypted);
        }
        Action::Prepare => {
            let processed =
                read_from(UNPROCESSED_FILENAME)
                .trim()
                .chars()
                .filter(|c| c == &' ' || c.is_ascii_alphabetic())
                .collect::<Vec<char>>()
                .chunks(EXPECTED_KEY_LENGTH)
                .map(|c| c.iter().collect::<String>())
                .collect::<Vec<String>>()
                .join("\n")
                .to_lowercase();

            write_to(PLAINTEXT_FILENAME, processed);
        }
        Action::Crack => {
            let crypted = read_from(CIPHER_FILENAME)
                .trim()
                .chars()
                .enumerate()
                .filter(|(i, _)| i % 33 != 32) // because not every \n in crypt is \n in plain ;)
                .map(|(_, c)| c)
                .collect::<String>()
                ;

            let decrypted = xor::crack(&crypted, EXPECTED_KEY_LENGTH)
                .chars()
                .collect::<Vec<char>>()
                .chunks(EXPECTED_KEY_LENGTH)
                .map(|c| c.iter().collect::<String>())
                .collect::<Vec<String>>()
                .join("\n")
                ;

            write_to(DECRYPTED_FILENAME, decrypted + "\n");
        }
    };
    // TODO: other todos
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
        "-p" => Ok(Action::Prepare),
        "-k" => Ok(Action::Crack),
        _ => Err("unknown action, try *one* of '-ekp' instead"),
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

fn exit_err<T: Display, U: Display>(message: T, context: U) -> !
{
    let _ = writeln!(&mut io::stderr(), "ERROR at {}: {}", context, message);
    process::exit(1);
}
