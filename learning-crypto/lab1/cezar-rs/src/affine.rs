use std::fmt;

extern crate phf;
use phf::phf_map;


#[derive(Debug)]
pub struct Key {
    pub a: i32, // the multiplicative coefficient
    pub b: i32, // the additive coefficient
}

pub const ALPHABET_LEN: i32 = 26;
pub static A_INVERSES: phf::Map<i32, i32> = phf_map! {
    1i32 => 1,
    3i32 => 9,
    5i32 => 21,
    7i32 => 15,
    9i32 => 3,
    11i32 => 19,
    15i32 => 7,
    17i32 => 23,
    19i32 => 11,
    21i32 => 5,
    23i32 => 17,
    25i32 => 25,
};

impl Key {
    fn invert(&self) -> Option<Self>
    {
        Some(Key { a: A_INVERSES.get(&self.a)?.clone(), b: -self.b })
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.a, self.b)
    }
}

// TODO: reduce duplication
fn encrypt_char(c: char, key: &Key) -> char {
    if !c.is_ascii_alphabetic() {
        return c;
    }
    let case_offset = if c.is_uppercase() {'A'} else {'a'} as u8;
    let c_value = ( c as u8 - case_offset ) as i32;

    let c_value_crypted = (key.a * c_value + key.b) % ALPHABET_LEN;

    (c_value_crypted as u8 + case_offset) as char
}

fn decrypt_char(c: char, inverted_key: &Key) -> char {
    if !c.is_ascii_alphabetic() {
        return c;
    }
    let case_offset = if c.is_uppercase() {'A'} else {'a'} as u8;
    let c_value = ( c as u8 - case_offset ) as i32;

    let mut c_value_plain = (inverted_key.a * (c_value + inverted_key.b)) % ALPHABET_LEN;

    if c_value_plain < 0 {
        c_value_plain += ALPHABET_LEN;
    }

    (c_value_plain as u8 + case_offset) as char
}

fn validate_key(key: Key) -> Result<Key, &'static str>
{
    match key.invert()
    {
        Some(_) => Ok(key),
        None => Err("Invalid key - there is no inversion for 'a'"),
    }
}


pub fn encrypt(plaintext: &str, key: &Key) -> String
{
    plaintext.chars()
        .map(|c| encrypt_char(c, key)).collect::<String>()
}

pub fn decrypt(ciphertext: &str, key: &Key) -> String
{
    let inverted_key = key.invert().unwrap();

    ciphertext.chars()
        .map(|c| decrypt_char(c, &inverted_key)).collect::<String>()
}

pub fn parse_key(raw_key: &str) -> Result<Key, &'static str>
{
    let key_tokens: Vec<&str> = raw_key.trim().split(" ").collect();
    if key_tokens.len() != 2 {
        return Err("Invalid key components count - 2 expected");
    }

    let _a = match key_tokens[0].parse::<i32>() {
        Ok(key) => Ok(key),
        Err(_) => Err("Failed to parse fist key component - are you sure it's a valid positive integer?"),
    };

    let _b = match key_tokens[1].parse::<i32>() {
        Ok(key) => Ok(key),
        Err(_) => Err("Failed to parse second key component - are you sure it's a valid positive integer?"),
    };

    let (a, b) = (_a?, _b?);

    let key_candidate = Key { a: a, b: b };

    validate_key(key_candidate)
}

pub fn infer_key(known_plaintext: &str, ciphertext: &str) -> Result<Key, &'static str>
{
    let (_x1, _x2) = (known_plaintext.chars().nth(0).unwrap() as i32, known_plaintext.chars().nth(1).unwrap() as i32);
    let (_y1, _y2) = (ciphertext.chars().nth(0).unwrap() as i32, ciphertext.chars().nth(1).unwrap() as i32);

    let ascii_offset = 'a' as i32;
    let (x1, x2, y1, y2) = (_x1 - ascii_offset, _x2 - ascii_offset, _y1 - ascii_offset, _y2 - ascii_offset);

    let _inverse = match A_INVERSES.get(&(x2 - x1)) {
        Some(x) => Ok(x),
        None => Err("Cannot compute inverse of plaintext difference"),
    };

    let mut a = _inverse? * (y2 - y1);
    let b = y1 - (a * x1);

    while a < 0
    {
        a += ALPHABET_LEN;
    };

    let key_candidate = Key { a: a % ALPHABET_LEN, b: b % ALPHABET_LEN};

    validate_key(key_candidate)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_nothing() {
        assert_eq!(encrypt("", &Key{ a: 1, b: 0}), "");
        assert_eq!(encrypt("", &Key{ a: 5, b: 3}), "");
        assert_eq!(encrypt("cleartext", &Key{ a: 1, b: 0}), "cleartext");
    }

    #[test]
    fn test_encrypt_something() {
        assert_eq!(encrypt("AFFINECIPHERaffinecipher", &Key{ a: 5, b: 8 }), "IHHWVCSWFRCPihhwvcswfrcp");
    }

    #[test]
    fn test_decrypt_nothing() {
        let msg = "";
        let key = Key { a: 3, b: 7 };
        assert_eq!(decrypt(&encrypt(msg, &key), &key), msg)
    }

    #[test]
    fn test_decrypt_something() {
        let key = Key { a: 5, b: 8 };

        let _ = decrypt("C", &key);
        assert_eq!(decrypt("I", &key), "A")
    }

    #[quickcheck]
    fn test_decrypt(msg: String) {
        let key = Key { a: 7, b: 15 };
        assert_eq!(decrypt(&encrypt(&msg, &key), &key), msg)
    }
}
