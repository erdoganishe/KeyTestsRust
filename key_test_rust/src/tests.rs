pub fn monobit_test(bits: &Vec<u8>)->bool {
    let count_of_zero = bits.iter().filter(|&&bit| bit == 0).count() as f64;
    (count_of_zero>=9654.0) & (count_of_zero<=10346.0)
}
