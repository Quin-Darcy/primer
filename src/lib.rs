use rand::Rng;
use num_bigint::{BigUint};


pub fn get_rand_nbit(num_of_bits: u32) -> BigUint {
    let mut rng = rand::thread_rng();
    let mut rand_bits: Vec<u8> = Vec::new();
    
    rand_bits.push(1_u8);
    for _ in 0..(num_of_bits-2 as usize) {
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
        println!("{:?}", get_rand_nbit(num_of_bits));
    }
}
