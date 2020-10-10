#[derive(Debug, PartialEq)]
pub struct Key {
    v: Vec<u8>,
}

impl Key {
    pub fn parse(raw_key: &str) -> Result<Key, &'static str>
    {
        Ok(Key{ v: raw_key.as_bytes().to_vec() })
    }
}

pub fn encrypt(plaintext: &str,  key: &Key) -> String
{
    String::from_utf8(
        plaintext
            .chars()
                .zip(key.v.iter().cycle())
                .map(|(c, k)| {
                    c as u8 ^ k
                })
        .collect::<Vec<_>>()
    ).unwrap()
}

pub fn crack(ciphertext: &str, key_length: usize) -> String
{
    let key = subgroup_split(ciphertext, key_length)
        .iter()
            .map(|c| { crack_column(c) })
        .collect::<Vec<_>>();

    encrypt(ciphertext, &Key{ v: key })
}


fn crack_column(ciphertext: &Vec<u8>) -> u8
{
    // TODO: fallback -> store Nope and inconclusive and cross tests them!
 
    // TODO: refactor
    use SpaceMatching::*;

    let mut space_matching = Nope;
    let mut j = 0;
    for i in 0 .. (ciphertext.len() - 2 - 1)
    {
        j = i;
        space_matching = match_space(ciphertext[i], ciphertext[i + 1], ciphertext[i + 2]); 
        if space_matching != Nope && space_matching != Inconclusive
        {
            break;
        }
    }

    let encoded_space_index = j + match space_matching {
        First => 0,
        Second => 1,
        Third => 2,
        _ => 1234567 // TODO: handle this better, cannot decrypt - output dots xD
    };

    let encoded_space = ciphertext [encoded_space_index];
    
    ' ' as u8 ^ encoded_space
}


#[derive(Debug, PartialEq)]
enum SpaceMatching {
    First, Second, Third, Nope, Inconclusive
}


fn match_space(m1: u8, m2: u8, m3: u8) -> SpaceMatching
{
    use XorOutcome::*;
    use SpaceMatching::*;

    let xor12 = xor_elements(m1, m2);
    let xor23 = xor_elements(m2,m3);
    let xor13 = xor_elements(m1,m3);

    match xor12 {
        Letters => match xor23 {
            Letters => Nope,
            Same => Nope,
            Space => Third,
        }
        Same => match xor23 {
            Letters => Nope,
            Same => Inconclusive,
            Space => Inconclusive,
        }
        Space => match xor23 {
            Letters => First,
            Same => Inconclusive,
            Space => {
                if xor13 == Letters { Second } else { Inconclusive }
            },
        }
    }
}

#[derive(Debug, PartialEq)]
enum XorOutcome {
    Same, Letters, Space
}

const XORED_SPACE_MASK: u8 = 0b0100_0000;

fn xor_elements(a: u8, b: u8) -> XorOutcome
{
    let xor = a ^ b;

    if xor == 0
    {
        XorOutcome::Same
    }
    else if (xor & XORED_SPACE_MASK) == 0
    {
        XorOutcome::Letters
    }
    else
    {
        XorOutcome::Space
    }
}


fn subgroup_split(string: &str, count: usize) -> Vec<Vec<u8>>
{
    (0 .. count)
        .map(|g| {
            string
                .chars()
                .enumerate()
                .filter(|(i, _)| {
                    i % count == g
                })
                .map(|(_, c)| c as u8)
            .collect::<Vec<_>>()
        })
    .collect::<Vec<_>>()
}


#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[quickcheck]
    fn test_key_parsing(key_vector: Vec<u8>)
    {
        let k = Key { v: key_vector };
        assert_eq!(Key::parse(&k.to_string()).unwrap(), k);
    }

    #[test_case('a' as u8, ' ' as u8, XorOutcome::Space)]
    #[test_case('a' as u8, 'b' as u8, XorOutcome::Letters)]
    #[test_case('x' as u8, 'x' as u8, XorOutcome::Same)]
    fn test_xor_elements(a: u8, b: u8, result: XorOutcome)
    {
        assert_eq!(xor_elements(a, b), result)
    }

    #[quickcheck]
    fn test_encrypt(key_vector: Vec<u8>, plaintext_vector: Vec<u8>)
    {
        // TODO: prevent non-ascii characters
        let plaintext = String::from_utf8(plaintext_vector).unwrap();
        let original_plaintext = plaintext.clone();
        let k = Key { v: key_vector };

        assert_eq!(encrypt(encrypt(plaintext, &k), &k), original_plaintext);
    }
}
