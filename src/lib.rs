use rand::Rng;
use num_bigint::{BigUint};



const FIRST_PRIMES: [u32; 110] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29,
                                    31, 37, 41, 43, 47, 53, 59, 61, 67,
                                    71, 73, 79, 83, 89, 97, 101, 103,
                                    107, 109, 113, 127, 131, 137, 139,
                                    149, 151, 157, 163, 167, 173, 179,
                                    181, 191, 193, 197, 199, 211, 223,
                                    227, 229, 233, 239, 241, 251, 257,
                                    263, 269, 271, 277, 281, 283, 293,
                                    307, 311, 313, 317, 331, 337, 347, 
                                    349, 353, 359, 367, 373, 379, 383,
                                    389, 397, 401, 409, 419, 421, 431,  
                                    433, 439, 443, 449, 457, 461, 463,
                                    467, 479, 487, 491, 499, 503, 509, 
                                    521, 523, 541, 547, 557, 563, 569,  
                                    571, 577, 587, 593, 599, 601];


pub fn get_rand_nbit(num_of_bits: &u32) -> BigUint {
    let mut rng = rand::thread_rng();
    let mut rand_bits: Vec<u8> = Vec::new();
    
    rand_bits.push(1_u8);
    for _ in 0..((num_of_bits-2) as usize) {
        rand_bits.push(rng.gen_range(0..2) as u8);
    }
    rand_bits.push(1_u8);

    BigUint::from_radix_be(&rand_bits[..], 2).unwrap()
}

fn initial_div_test(candidate: BigUint, num_of_bits: u32) -> BigUint {
    let mut non_divisors: u32 = 0;
    let mut new_candidate: BigUint = candidate;
    loop {
        for p in FIRST_PRIMES {
            if (new_candidate.clone() % p).is_zero() && new_candidate > BigUint::from(p) {
                new_candidate = get_rand_nbit(num_of_bits);
                non_divisors = 0;
                break;
            } else {
                non_divisors += 1;
            }
        }
        if non_divisors as usize == FIRST_PRIMES.len() {
            return new_candidate;
        }
    }
}

fn get_two_pow(candidate: BigUint) -> u32 {
    let mut e: u32 = 0;
    let shift_size: usize = 1;
    let mut n: BigUint candidate.clone()-1_u32;

    while (&n % 2_u32).is_zero() {
        n >>= shift_size;
        e += 1;
    }
    return e;
}
/*
fn miller_rabin(candidate: BigUint) -> bool {
    let mut rng = rand::thread_rng();
    let two_pow: u32 = get_two_pow(candidate.clone());
    let rmdr: BigUint = candidate.clone() / (2_u32).pow(two_pow);
    let iters: usize = 20;

    for _ in 0..iters {
        let base: BigUint = rng.gen_biguint_range(&BigUint::from(2_u32), &(candidate.clone()-1_u32));
    }
}

pub fn get_large_prime(num_of_bits: u32) -> BigUint {
    let candidate: BigUint;
    loop {
        if 
    }
}
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn show_num() {
        let num_of_bits: u32 = 8;
        println!("{:?}", get_rand_nbit(&num_of_bits));
    }
}
