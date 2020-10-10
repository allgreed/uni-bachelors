use std::mem;


const BITS_IN_BYTE: usize = 8;


pub fn crypt_ecb(input: &Vec<bool>) -> Vec<bool>
{
    input // TODO: transmute ?
        .chunks(mem::size_of::<i64>() * BITS_IN_BYTE - 1)
        .map(|x| { // TODO: fold? o.0
            let mut acc: u64 = 0;
            let mut exponent: u64 = 1;
 
            for bit in x {
                acc += *bit as u64 * exponent;

                exponent *= 2;
            }
            acc
        })

        // crypting function
        .map(|x| {
            x ^ 0b10101010101001010101001
        })

        .map(|x| {
            let mut acc = Vec::with_capacity(mem::size_of::<u64>() * BITS_IN_BYTE);
            let mut mask: u64 = 1;

            for i in 0..mem::size_of::<u64>() * BITS_IN_BYTE - 1 {
                acc.push(if x & mask != 0 { true } else { false });

                mask *= 2;
            }

            acc
        })
        .fold(vec![], |mut acc, mut cur| {
            acc.append(&mut cur);

            acc
        })
}


pub fn crypt_cbc(input: &Vec<bool>) -> Vec<bool>
{
    let iv = 0;

    let shifted = input // TODO: transmute ?
        .chunks(mem::size_of::<i64>() * BITS_IN_BYTE - 1)
        .map(|x| { // TODO: fold? o.0
            let mut acc: u64 = 0;
            let mut exponent: u64 = 1;
 
            for bit in x {
                acc += *bit as u64 * exponent;

                exponent *= 2;
            }
            acc
        })
    .collect::<Vec<_>>();

    let mut crypted = Vec::with_capacity(shifted.len());
    let mut previous_encrypted_block = iv;

    for block in shifted {
        let encrypted_block = block ^ previous_encrypted_block ^ 0b10101010101001010101001;
        previous_encrypted_block = encrypted_block;

        crypted.push(encrypted_block);
    }

    crypted
        .iter()
        .map(|x| {
            let mut acc = Vec::with_capacity(mem::size_of::<u64>() * BITS_IN_BYTE);
            let mut mask: u64 = 1;

            for i in 0..mem::size_of::<u64>() * BITS_IN_BYTE - 1 {
                acc.push(if x & mask != 0 { true } else { false });

                mask *= 2;
            }

            acc
        })
        .fold(vec![], |mut acc, mut cur| {
            acc.append(&mut cur);

            acc
        })
}

// TODO: extract common logic for block splitting
//fn crypt_block(input: &Vec<bool>) -> Vec<bool>
