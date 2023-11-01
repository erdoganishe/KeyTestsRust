mod tests;
mod generator;

use tests::{monobit_test, max_series_length_test, poker_test, series_length_test};
use generator::generate_random_bits;



fn main() {
    let bits = generate_random_bits(20000);
    
    // for i in 0..bits.len() {
    //    print!("{}", bits[i]);
    // }
    // println!();
    println!("monobit test result: {}", monobit_test(&bits));
    println!("max series test result: {}", max_series_length_test(&bits));
    println!("Pokker test result: {}", poker_test(&bits, 4));
    println!("Pokker test result: {}", series_length_test(&bits));

}