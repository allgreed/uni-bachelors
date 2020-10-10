use crate::affine;


pub fn encrypt(plaintext: &str, key: i32) -> String
{
    affine::encrypt(plaintext, &affine::Key { a: 1, b: key })
}

pub fn decrypt(ciphertext: &str, key: i32) -> String
{
    encrypt(ciphertext, affine::ALPHABET_LEN - key)
}

pub fn parse_key(raw_key: &str) -> Result<i32, &'static str>
{
    match raw_key.trim().parse::<i32>() {
        Ok(key) => Ok(key),
        Err(_) => Err("Failed to parse key - are you sure it's a valid positive integer?"),
    }
}

pub fn infer_key(known_plaintext: &str, ciphertext: &str) -> i32
{
    let char_plain = known_plaintext.chars().nth(0).unwrap() as i32;
    let char_cipher = ciphertext.chars().nth(0).unwrap() as i32;

    (char_cipher - char_plain + affine::ALPHABET_LEN) % affine::ALPHABET_LEN
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_infer_key() {
        assert_eq!(infer_key("a", "a"), 0);
        assert_eq!(infer_key("a", "b"), 1);
        assert_eq!(infer_key("z", "a"), 1);
    }

    #[test]
    fn test_encrypt_nothing() {
        assert_eq!(encrypt("", 0), "");
        assert_eq!(encrypt("", 5), "");
    }

    #[test]
    fn test_encrypt_something() {
        assert_eq!(encrypt("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz", 13), "NOPQRSTUVWXYZABCDEFGHIJKLMnopqrstuvwxyzabcdefghijklm");
    }

    #[test]
    fn test_decrypt_nothing() {
        assert_eq!(decrypt(&encrypt("", 1), 1), "")
    }

    #[quickcheck]
    fn test_decrypt_something(s: String, c: u8) {
        assert_eq!(decrypt(&encrypt(&s, 8), 8), s)
    }
}

