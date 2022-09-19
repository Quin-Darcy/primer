#![allow(dead_code)]
#![allow(unused_imports)]
use rand::Rng;
use num_traits::{One};
use num_traits::identities::{Zero};
use num_traits::cast::ToPrimitive;
use num_bigint::{BigUint, RandBigInt};


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

// This function uses its argument to generate a num_of_bits-bit number.
// The generation includes randomly choosing the bit values. Note that
// the first and last bit are always one to assure the number has exactly
// num_of_bits many bits and is odd.
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

// This function receives a prime candidate and if the cadidate is divisble
// by any number in FIRST_PRIMES, then false is returned. The function returns
// true if it is equal to a prime in the list or if it is not divisible by any 
// of the primes in the list. 
fn initial_div_test(candidate: BigUint) -> bool {
    for p in FIRST_PRIMES {
        if (candidate.clone() % p).is_zero() && candidate >= BigUint::from(p) {
            if candidate == BigUint::from(p) {
                return true;
            } else {
                return false;
            }
        }  
    }
    return true;
}

// This function calculates the largest power of two that divides the
// argument and returns the exponent of the exponent. 
fn get_two_pow(candidate: BigUint) -> u32 {
    let mut e: u32 = 0;
    let shift_size: usize = 1;
    let mut n: BigUint = candidate.clone()-1_u32;

    while (&n % 2_u32).is_zero() {
        n >>= shift_size;
        e += 1;
    }
    return e;
}

// This function implements the Miller-Rabin primality test and returns
// true is the candidate is likely prime and false if it is composite. 
fn miller_rabin(candidate: BigUint) -> bool {
    let iters: usize = 20;
    let mut rng = rand::thread_rng();
    let one: BigUint = BigUint::one();
    let neg_one: BigUint = candidate.clone()-1_u32;
    let two_pow: u32 = get_two_pow(candidate.clone());
    let rmdr: BigUint = candidate.clone() / (2_u32).pow(two_pow);
    let mut base: BigUint = rng.gen_biguint_range(&BigUint::from(2_u32), &(candidate.clone()-1_u32));

    // First check
    base = BigUint::modpow(&base, &rmdr.clone(), &candidate.clone());
    if base == one  || base == neg_one {
        return true;
    } 
    
    // Remaining checks
    for _ in 0..iters {
        base = BigUint::modpow(&base, &BigUint::from(2_u32), &candidate.clone());       
        if base == one {
            return false;
        } else if base == neg_one {
            return true;
        } 
    }
    return false;
}

// This function returns a num_of_bits-bit prime number. 
pub fn get_large_prime(num_of_bits: mut u32) -> u128 {
    if num_of_bits > 128 {
      num_of_bits = 128_u32;
    }
    let mut candidate = get_rand_nbit(&num_of_bits);

    loop {
        if initial_div_test(candidate.clone()) {
            if miller_rabin(candidate.clone()) {
                return candidate.to_u128().unwrap();
            } else {
                candidate = get_rand_nbit(&num_of_bits);
            }
        } else {
            candidate = get_rand_nbit(&num_of_bits);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_div_tests() {
        assert_eq!(initial_div_test(BigUint::from(53_u32)), true);
        assert_eq!(initial_div_test(BigUint::from(10000000000_u64)), false);
    }

    #[test]
    fn highest_two_pow() {
        assert_eq!(get_two_pow(BigUint::from(33_u32)), 5_u32);
        assert_eq!(get_two_pow(BigUint::from(602_u32)), 0);
    }

    #[test]
    fn miller_rabin_test() {
        assert_eq!(miller_rabin(BigUint::from(53_u32)), true);
        assert_eq!(miller_rabin(BigUint::from(123456789_u64)), false);
        assert_eq!(miller_rabin(BigUint::from(205403002746320107182300608701_u128)), true);
    }
}
