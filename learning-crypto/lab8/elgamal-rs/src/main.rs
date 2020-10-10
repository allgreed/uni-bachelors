extern crate num_bigint as bigint;

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
use bigint::{ToBigInt, RandBigInt, BigInt};


mod elgamal;


const SEED_FILENAME:&str = "elgamal.txt";
const PUBKEY_FILENAME:&str = "public.txt";
const PRIVKEY_FILENAME:&str = "private.txt";
const PLAINTEXT_FILENAME:&str = "plain.txt";
const ENCRYPTED_FILENAME:&str = "crypto.txt";
const DECRYPTED_FILENAME:&str = "decrypt.txt";
const MSG_FILENAME:&str = "message.txt ";
const SIG_FILENAME:&str = "signature.txt ";
const VERIFY_FILENAME:&str = "verify.txt ";


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
        Action::Keygen => {
            let raw_seed: Vec<String> = read_from(SEED_FILENAME).trim().split(' ').map(|s| s.to_string()).collect();

            let p = &raw_seed[0].trim().parse::<BigInt>()
                .unwrap_or_else(|e| exit_err(e, "parsing seed"));
            let g = &raw_seed[1].trim().parse::<BigInt>()
                .unwrap_or_else(|e| exit_err(e, "parsing seed"));

            let power = 1;
            let exponent = 2;
            // TODO: generate key

            write_to(PRIVKEY_FILENAME, format!("{}\n{}\n{}\n", p, g, exponent));
            write_to(PUBKEY_FILENAME, format!("{}\n{}\n{}\n", p, g, power));
        }
        Action::Encrypt => {
            let raw_key: Vec<String> = read_from(PUBKEY_FILENAME).trim().split(' ').map(|s| s.to_string()).collect();

            let p = &raw_key[0].trim().parse::<BigInt>()
                .unwrap_or_else(|e| exit_err(e, "parsing key"));
            let g = &raw_key[1].trim().parse::<BigInt>()
                .unwrap_or_else(|e| exit_err(e, "parsing key"));
            let power = &raw_key[2].trim().parse::<BigInt>()
                .unwrap_or_else(|e| exit_err(e, "parsing key"));

            let plain = read_from(PLAINTEXT_FILENAME).trim().to_string();
            // TODO: Jeśli warunek m<p nie jest spełniony, sygnalizuje błąd. 
 
            // TODO: this - encryption method
            write_to(ENCRYPTED_FILENAME, vigenere::encrypt(&plaintext, &key) + "\n");
        }
        Action::Decrypt => {
            let raw_key: Vec<String> = read_from(PRIVKEY_FILENAME).trim().split(' ').map(|s| s.to_string()).collect();

            let p = &raw_key[0].trim().parse::<BigInt>()
                .unwrap_or_else(|e| exit_err(e, "parsing key"));
            let g = &raw_key[1].trim().parse::<BigInt>()
                .unwrap_or_else(|e| exit_err(e, "parsing key"));
            let exponent = &raw_key[2].trim().parse::<BigInt>()
                .unwrap_or_else(|e| exit_err(e, "parsing key"));
 
            let crypto = read_from(ENCRYPTED_FILENAME).trim().to_string();
 
            // TODO: this - decryption method
            write_to(DECRYPTED_FILENAME, );
        }
        Action::Sign => {
            let raw_key: Vec<String> = read_from(PRIVKEY_FILENAME).trim().split(' ').map(|s| s.to_string()).collect();

            let p = &raw_key[0].trim().parse::<BigInt>()
                .unwrap_or_else(|e| exit_err(e, "parsing key"));
            let g = &raw_key[1].trim().parse::<BigInt>()
                .unwrap_or_else(|e| exit_err(e, "parsing key"));
            let exponent = &raw_key[2].trim().parse::<BigInt>()
                .unwrap_or_else(|e| exit_err(e, "parsing key"));
 
            let msg = read_from(MSG_FILENAME).trim().to_string();

            // TODO: this
            
            // TODO: podpis, czyli dwa wiersze zapisane do pliku signature.txt. 
        }
        Action::Verify => {
            let raw_key: Vec<String> = read_from(PUBKEY_FILENAME).trim().split(' ').map(|s| s.to_string()).collect();

            let p = &raw_key[0].trim().parse::<BigInt>()
                .unwrap_or_else(|e| exit_err(e, "parsing key"));
            let g = &raw_key[1].trim().parse::<BigInt>()
                .unwrap_or_else(|e| exit_err(e, "parsing key"));
            let power = &raw_key[2].trim().parse::<BigInt>()
                .unwrap_or_else(|e| exit_err(e, "parsing key"));

            let msg = read_from(MSG_FILENAME).trim().to_string();
            let sig = read_from(SIG_FILENAME).trim().to_string();

            // TODO: this
 
            // TODO: Wynik weryfikacji (T/N) jest wyświetlany na ekranie oraz jest zapisywany w pliku verify.txt. 
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
        "-s" => Ok(Action::Sign),
        "-v" => Ok(Action::Verify),
        "-k" => Ok(Action::Keygen),
        _ => Err("unknown action, try *one* of '-edsvk' instead"),
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
