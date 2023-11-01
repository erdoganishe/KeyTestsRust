mod tests;

use rand::Rng;
use tests::{monobit_test};

fn generate_random_bits(length: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut random_bits = Vec::with_capacity(length);
    
    for _ in 0..length {
        let random_bit = rng.gen_range(0..=1);
        random_bits.push(random_bit);
    }
    
    random_bits
}

fn main() {
    let bits = generate_random_bits(20000);
    
    for i in 0..bits.len() {
       print!("{}", bits[i]);
    }
    println!();
    println!("monobit test result: {}", monobit_test(&bits));
}