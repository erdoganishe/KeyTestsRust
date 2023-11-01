mod tests;
mod generator;

use tests::{monobit_test, max_series_length_test, poker_test, series_length_test};
use generator::generate_random_bits;



fn main() {
    let bits = generate_random_bits(20000);
    
    let results = [monobit_test(&bits), max_series_length_test(&bits), poker_test(&bits, 4),series_length_test(&bits)];
    println!("Monobit test result: {}", results[0]);
    println!("Max series test result: {}", results[1]);
    println!("Pokker test result: {}", results[2]);
    println!("Series test result: {}", results[3]);
    if (results.iter().filter(|&&res|res == false).count() > 0){
        println!("20 000 bites aren`t random enough.");
    }
    else {
        println!("20 000 bites are random enough.");
    }

}