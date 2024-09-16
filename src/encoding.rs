pub fn augmented_hadamard_matrix() -> [u64; 128] {
    let mut matrix: [u64; 128] = [0; 128];
    for (i, item) in matrix.iter_mut().enumerate() {
        for j in 0..64 {
            let bits = i & j;
            if (bits.count_ones() & 0x1 == 0) == (i & 0x40 == 0) {
                *item |= 0x1 << (63 - j);
            }
        }
    }
    matrix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hadamard_rows() {
        let hadamard = augmented_hadamard_matrix();
        let row_0 = hadamard[0];
        assert_eq!(row_0, u64::MAX);
        let row_1 = hadamard[1];
        assert_eq!(row_1, 0xaaaa_aaaa_aaaa_aaaa);
    }

    #[test]
    fn hadamard_rows_and() {
        let hadamard = augmented_hadamard_matrix();
        for (index, row) in hadamard.iter().enumerate() {
            for (other_index, other_row) in hadamard.iter().enumerate() {
                let bits = *row ^ *other_row;
                if index == other_index {
                    assert_eq!(bits.count_ones(), 0)
                } else {
                    assert!(
                        bits.count_ones() >= 32,
                        "Got {} set bits at index {index}, other index {other_index}",
                        bits.count_ones()
                    );
                }
            }
        }
    }
}
