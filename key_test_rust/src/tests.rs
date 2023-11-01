pub fn monobit_test(bits: &Vec<u8>)->bool {
    let count_of_zero = bits.iter().filter(|&&bit| bit == 0).count() as f64;
    (count_of_zero>=9654.0) & (count_of_zero<=10346.0)
}
pub fn max_series_length_test(bits: &[u8]) -> bool {
    let mut current_series_length = 0;
    let mut max_zero_series_length = 0;
    let mut max_one_series_length = 0;
    let mut prev_bit = -1; 

    for &bit in bits {
        if bit == 0 {
            if prev_bit == 0 {
                current_series_length += 1;
            } else {
                current_series_length = 1;
            }
            prev_bit = 0;
            max_zero_series_length = max_zero_series_length.max(current_series_length);
        } else if bit == 1 {
            if prev_bit == 1 {
                current_series_length += 1;
            } else {
                current_series_length = 1;
            }
            prev_bit = 1;
            max_one_series_length = max_one_series_length.max(current_series_length);
        } else {
            current_series_length = 0;
            prev_bit = 2;
        }
    }

    max_zero_series_length <= 36 && max_one_series_length <= 36
}
