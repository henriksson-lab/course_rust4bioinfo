/// Same packing as exercise 2 (re-implemented here so each exercise stands alone).
fn pack(s: &[u8]) -> u64 {
    let mut acc = 0u64;
    for (i, &b) in s.iter().enumerate() {
        let v: u64 = match b {
            b'A' => 0,
            b'C' => 1,
            b'G' => 2,
            b'T' => 3,
            _ => 0,
        };
        acc |= v << (2 * i);
    }
    acc
}

/// Hamming distance on two ACGT strings of equal length - count positions where they differ.
fn hamming_naive(a: &[u8], b: &[u8]) -> u32 {
    assert_eq!(a.len(), b.len());
    a.iter().zip(b.iter()).filter(|(x, y)| x != y).count() as u32
}

/// Hamming distance via XOR + popcount on packed sequences.
/// `a` and `b` must already be packed and represent sequences of the SAME length.
fn hamming_packed(a: u64, b: u64) -> u32 {
    // TODO: XOR a and b to find mismatched 2-bit slots, then count one bit per slot.
    //
    // Naive idea: (a ^ b).count_ones() / 2.
    //   This is WRONG for the encoding A=00, C=01, G=10, T=11, because e.g. A vs C
    //   only differs in 1 bit (00 ^ 01 = 01), so /2 truncates to 0.
    //
    // Correct kernel: fold each 2-bit slot to a single "any-mismatch" bit, then count.
    //   let m = a ^ b;
    //   ((m | (m >> 1)) & 0x5555_5555_5555_5555).count_ones()
    //
    //   0x5555_5555_5555_5555 is 0101...0101 in binary - one mask bit per 2-bit slot.
    let _ = (a, b);
    0
}

fn main() {
    let s1 = b"ACGTACGTACGT";
    let s2 = b"ACGTACGAACGA";
    let h_naive = hamming_naive(s1, s2);
    let h_packed = hamming_packed(pack(s1), pack(s2));
    println!("naive  = {}", h_naive);
    println!("packed = {}", h_packed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_zero() {
        assert_eq!(hamming_packed(pack(b"ACGT"), pack(b"ACGT")), 0);
    }

    #[test]
    fn one_diff() {
        // last base C vs G
        assert_eq!(hamming_packed(pack(b"ACGC"), pack(b"ACGG")), 1);
    }

    #[test]
    fn many_diffs_match_naive() {
        let s1 = b"ACGTACGTACGTACGTACGT"; // 20 bases
        let s2 = b"TCGAACGTACATACGTACTT";
        assert_eq!(
            hamming_packed(pack(s1), pack(s2)),
            hamming_naive(s1, s2)
        );
    }

    #[test]
    fn all_diff() {
        // ACGT vs TGCA - all 4 differ
        assert_eq!(hamming_packed(pack(b"ACGT"), pack(b"TGCA")), 4);
    }

    #[test]
    fn random_seeds() {
        // Compare naive vs packed on a few fixed strings.
        let pairs: &[(&[u8], &[u8])] = &[
            (b"AAAAAAAA", b"AAAAAAAC"),
            (b"GGGGGGGG", b"AAAAAAAA"),
            (b"ACGTACGT", b"GCATGCAT"),
        ];
        for &(a, b) in pairs {
            assert_eq!(
                hamming_packed(pack(a), pack(b)),
                hamming_naive(a, b),
                "naive and packed disagreed for {} vs {}",
                std::str::from_utf8(a).unwrap(),
                std::str::from_utf8(b).unwrap(),
            );
        }
    }
}
