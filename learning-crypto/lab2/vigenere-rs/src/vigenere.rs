use std::fmt;

use crate::caesar;

const INDEX_OF_COINCIDENCE_EPSILON: f64 = 0.004;
const ENGLISH_INDEX_OF_COINCIDENCE: f64 = 0.066;


#[derive(Debug)]
pub struct Key {
    pub v: Vec<i32>,
}


pub fn encrypt(plaintext: &str, key: &Key) -> String
{
    transform(plaintext, key, caesar::encrypt)
}

pub fn decrypt(ciphertext: &str, key: &Key) -> String
{
    transform(ciphertext, key, caesar::decrypt)
}

pub fn crack(ciphertext: &str) -> Key
{
    let mut distributions:Vec<Vec<f64>> = vec![vec![]];

    for i in 1 .. ciphertext.len()
    {
        distributions =
            subgroup_split(ciphertext, i as usize)
                .iter()
                    .map(|s| characters_distribution(&s))
                .collect::<Vec<_>>()
            ;

        let (sum, count) =
            distributions
                .iter()
                    .map(|d| index_of_coincidence(d))
                    .fold((0.0, 0), |acc, cur| (acc.0 + cur, acc.1 + 1))
            ;

        let avg_index_of_coincidence = sum / count as f64;

        if (avg_index_of_coincidence - ENGLISH_INDEX_OF_COINCIDENCE).abs() < INDEX_OF_COINCIDENCE_EPSILON
        {
            break;
        }
    }

    let key_components = 
        distributions
            .iter()
                .map(|ds| ds.iter().enumerate().max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap()).unwrap().0)
                .map(|i| ((i as u8 - ('e' as u8 - 'a' as u8) + 26) % 26) as i32)
            .collect::<Vec<_>>();

    Key { v: key_components }
}

fn index_of_coincidence(frequencies: &Vec<f64>) -> f64
{
    frequencies
        .iter()
            .map(|x| x.powf(2.0))
        .sum()
}

fn subgroup_split(string: &str, count: usize) -> Vec<String>
{
    (0 .. count)
        .map(|g| {
            string
                .chars()
                .enumerate()
                .filter(|(i, _)| {
                    i % count == g
                })
                .map(|(_, c)| c)
            .collect::<String>()
        })
    .collect::<Vec<_>>()
}

fn characters_distribution(string: &str) -> Vec<f64>
{
    let string_length = string.chars().count();

    ('a' as u8 .. ('z' as u8) + 1)
        .map(|c| character_frequency(c as char, string, string_length))
        .collect::<Vec<_>>()
}

fn character_frequency(character: char, string: &str, string_length: usize) -> f64
{
    string
        .chars()
        .filter(|c| c == &character)
        .count() as f64
        / string_length as f64
}

fn transform<F>(input: &str, key: &Key, mut f: F) -> String
where F: FnMut(char, i32) -> char {
    input
        .chars()
        .zip(key.v.iter().cycle())
        .map(|(c, k)| f(c, *k))
        .collect::<String>()
}


impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.v.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(" "))
    }
}

impl Key {
    pub fn parse(raw_key: &str) -> Result<Key, &'static str>
    {
        let v = raw_key
            .trim()
            .split(" ")
            .map(|token| {
                token.parse::<i32>().or(Err("Failed to parse key component - are you sure it's a valid integer?")) 
            })
            .collect::<Result<Vec<_>, &'static str>>();

        Ok(Key{ v: v? })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character_frequency()
    {
        assert_eq!(character_frequency('a', "abcd", 4), 0.25);
    }
}
