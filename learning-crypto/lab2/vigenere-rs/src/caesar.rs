pub const ALPHABET_LEN: i32 = 26;
pub const ASCII_LETTER_OFFSET: i32 = 'a' as i32;


pub fn encrypt(c: char, key: i32) -> char
{
    debug_assert!(c.is_ascii_alphabetic() && c.is_lowercase());
 
    let c_encrypted = (c as i32 - ASCII_LETTER_OFFSET + key) % ALPHABET_LEN;
    ((c_encrypted + ASCII_LETTER_OFFSET) as u8) as char
}

pub fn decrypt(c: char, key: i32) -> char
{
    let key_inverse = ALPHABET_LEN - ( key % ALPHABET_LEN);
    encrypt(c, key_inverse)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[quickcheck]
    fn test_encryption(s: u8) {
        let key = s as i32;

        for i in 0..26
        {
            let c = ('a' as u8 + i) as char;
            assert_eq!(decrypt(encrypt(c, key), key), c)
        }
    }
}
