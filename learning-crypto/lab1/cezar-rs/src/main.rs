use std::io::prelude::*;
use std::io::{self, Write};
use std::{process, env};
use std::fmt::Display;
use std::fs::File;

mod caesar;
mod affine;

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;


const PLAINTEXT_FILENAME:&str = "plain.txt";
const CIPHER_FILENAME:&str = "crypto.txt";
const DECRYPTED_FILENAME:&str = "decrypt.txt";
const KEY_FILENAME:&str = "key.txt";
const KEY_RECOVERED_FILENAME:&str = "key-new.txt";
const KNOWN_PLAINTEXT_FILENAME:&str = "extra.txt";


#[derive(Debug)]
enum Cipher {
    Caesar, Affine
}

#[derive(Debug)]
enum Action {
    Encrypt, Decrypt, CrackKnownPlaintext, CrackUnknown
}


fn main() {
    let argv: Vec<String> = env::args().collect();
    let args = &argv[1 .. ];
    
    let (cipher, action) = parse_arguemnts(args)
        .unwrap_or_else(|e| exit_err(e, "parsing arguments"));

    match action {
        Action::Encrypt => {
            let plaintext = read_from(PLAINTEXT_FILENAME);
            let raw_key = read_from(KEY_FILENAME);
            
            let ciphertext = match cipher {
                Cipher::Caesar => {
                    let key = caesar::parse_key(&raw_key)
                        .unwrap_or_else(|e| exit_err(e, "parsing key"));

                    caesar::encrypt(&plaintext, key)
                }
                Cipher::Affine => {
                    let key = affine::parse_key(&raw_key)
                        .unwrap_or_else(|e| exit_err(e, "parsing key"));

                    affine::encrypt(&plaintext, &key)
                }
            };

            write_to(CIPHER_FILENAME, ciphertext);
        }
        Action::Decrypt => {
            let ciphertext = read_from(CIPHER_FILENAME);
            let raw_key = read_from(KEY_FILENAME);

            let plaintext = match cipher {
                Cipher::Caesar => {
                    let key = caesar::parse_key(&raw_key)
                        .unwrap_or_else(|e| exit_err(e, "parsing key"));

                    caesar::decrypt(&ciphertext, key)
                }
                Cipher::Affine => {
                    let key = affine::parse_key(&raw_key)
                        .unwrap_or_else(|e| exit_err(e, "parsing key"));

                    affine::decrypt(&ciphertext, &key)
                }
            };
            
            write_to(DECRYPTED_FILENAME, plaintext);
        }
        Action::CrackUnknown => {
            let ciphertext = read_from(CIPHER_FILENAME);
            let mut plaintext_candidates = Vec::new();

            match cipher {
                Cipher::Caesar => {
                    for i in 1..affine::ALPHABET_LEN {
                        let key = i;

                        let plaintext_candidate = caesar::decrypt(&ciphertext, key);
                        plaintext_candidates.push(plaintext_candidate);
                    }
                }
                Cipher::Affine => {
                    for a in affine::A_INVERSES.keys() {
                        for b in 0..affine::ALPHABET_LEN {
                            let key = affine::Key{ a: *a, b: b };

                            let plaintext_candidate = affine::decrypt(&ciphertext, &key);
                            plaintext_candidates.push(plaintext_candidate);
                        }
                    }
                }
            }
            
            write_to(DECRYPTED_FILENAME, plaintext_candidates.join(""));
        }
        Action::CrackKnownPlaintext => {
            let known_plaintext = read_from(KNOWN_PLAINTEXT_FILENAME);
            let ciphertext = read_from(CIPHER_FILENAME);
            
            let plaintext = match cipher {
                Cipher::Caesar => {
                    let key = caesar::infer_key(&known_plaintext, &ciphertext);

                    write_to(KEY_RECOVERED_FILENAME, format!("{}\n", key));
                    caesar::decrypt(&ciphertext, key)
                },
                Cipher::Affine => {
                    let key = affine::infer_key(&known_plaintext, &ciphertext)
                        .unwrap_or_else(|e| exit_err(e, "recovering key"));

                    write_to(KEY_RECOVERED_FILENAME, format!("{}\n", key));
                    affine::decrypt(&ciphertext, &key)
                },
            };

            write_to(DECRYPTED_FILENAME, plaintext);
        }
    };
}


fn parse_arguemnts(args: &[String]) -> Result<(Cipher, Action), &'static str>
{
    if args.len() != 2 {
        return Err("wrong number of arguments - exactly 2 arguments were expected");
    }
    let (flag0, flag1) = (&args[0], &args[1]);

    let (cipher, action_flag) = match flag_to_cipher(flag0) {
        Ok(x) => (x, flag1),
        Err(_) => (flag_to_cipher(flag1)?, flag0),
    };
    let action = flag_to_action(action_flag)?;

    Ok((cipher, action))
}


fn flag_to_cipher(flag: &str) -> Result<Cipher, &'static str> {
    match flag {
        "-c" => Ok(Cipher::Caesar),
        "-a" => Ok(Cipher::Affine),
        _ => Err("unknown cipher, try '-c' or '-a' instead"),
    }
}

fn flag_to_action(flag: &str) -> Result<Action, &'static str> {
    match flag {
        "-e" => Ok(Action::Encrypt),
        "-d" => Ok(Action::Decrypt),
        "-j" => Ok(Action::CrackKnownPlaintext),
        "-k" => Ok(Action::CrackUnknown),
        _ => Err("unknown action, try *one* of '-edjk' instead"),
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
