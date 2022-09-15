use rand::Rng;
use num_bigint::{BigUint};



const first_primes_list: [u32; 110] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29,
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


fn initial_div_test(candidate: BigUint, num_of_bits: u32) -> BigUint {
    loop {
        for prime in first_primes_list {
            if (candidate % prime).if_zero() && prime.pow(2_u32) <= candidate {
                break;
            } else {
                candidate
            }
        }
        candidate = get_rand_nbit(&num_of_bits);
    }
}

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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn show_num() {
        let num_of_bits: u32 = 8;
        println!("{:?}", get_rand_nbit(&num_of_bits));
    }
}
