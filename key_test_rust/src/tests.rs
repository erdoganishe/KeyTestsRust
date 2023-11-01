pub fn monobit_test(bits: &Vec<u8>)->bool {
    let count_of_zero = bits.iter().filter(|&&bit| bit == 0).count() as f64;
    (count_of_zero>=9654.0) && (count_of_zero<=10346.0)
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

pub fn poker_test(bits: &[u8], m: usize) -> bool {
    let k = bits.len() / m; 
    let mut block_counts = vec![0; 1 << m]; 

    for i in 0..k {
        let block = (0..m).fold(0, |acc, j| acc | (bits[i * m + j] << (m - j - 1)));
        block_counts[block as usize] += 1;
    }

    let mut x3 = 0.0;
    for i in 0..(1 << m) {
        x3 += f64::from(block_counts[i] * block_counts[i]);
    }

    x3 = (x3 * (1 << m) as f64 / k as f64) - k as f64;
   // println!("x3: {}", x3);
    (x3 >= 1.03) && (x3 <= 57.4)
}