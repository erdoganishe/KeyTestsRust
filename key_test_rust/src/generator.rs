use rand::Rng;

pub fn generate_random_bits(length: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut random_bits = Vec::with_capacity(length);
    
    for _ in 0..length {
        let random_bit = rng.gen_range(0..=1);
        random_bits.push(random_bit);
    }
    
    random_bits
}