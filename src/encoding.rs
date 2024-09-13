pub fn augmented_hadamard_matrix() -> [u64; 128] {
    let mut matrix: [u64; 128] = [0; 128];
    for i in 0..matrix.len() {
        for j in 0..64 {
            let bits = i & j;
            if (bits.count_ones() & 0x1 == 0) == (i & 0x40 == 0) {
                matrix[i] = matrix[i] | (0x1 << (63 - j));
            }
        }
    }
    matrix
}
