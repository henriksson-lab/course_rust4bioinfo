/// Same packing as exercise 2 (re-implemented here so each exercise stands alone).
fn pack(s: &[u8]) -> u64 {
    let mut acc = 0u64;
    for (i, &b) in s.iter().enumerate() {
        let v: u64 = match b { b'A' => 0, b'C' => 1, b'G' => 2, b'T' => 3, _ => 0 };
        acc |= v << (2 * i);
    }
    acc
}

fn unpack(packed: u64, len: usize) -> Vec<u8> {
    (0..len).map(|i| b"ACGT"[((packed >> (2 * i)) & 0b11) as usize]).collect()
}

/// Reverse the *2-bit pairs* of `x`, treating it as a sequence of `len` bases.
/// Example: ACGT packed is [A][C][G][T] in low-to-high bits. Reversing the
/// pairs gives [T][G][C][A] in low-to-high bits.
fn reverse_pairs(x: u64, len: usize) -> u64 {
    // TODO: extract each 2-bit pair from x and place it in the mirrored position.
    //   for i in 0..len:
    //     let pair = (x >> (2*i)) & 0b11;
    //     result |= pair << (2 * (len - 1 - i));
    let _ = (x, len);
    0
}

/// Reverse-complement of a packed sequence of length `len`.
///   1. complement: XOR with all-1s in the low (2*len) bits.
///   2. reverse the 2-bit pairs.
fn revcomp(x: u64, len: usize) -> u64 {
    // TODO:
    //   let mask = if len == 32 { u64::MAX } else { (1u64 << (2 * len)) - 1 };
    //   let comp = x ^ mask;
    //   reverse_pairs(comp, len)
    let _ = (x, len);
    0
}

fn main() {
    let s = b"ACGTACGT";
    let p = pack(s);
    let rc = revcomp(p, s.len());
    println!("orig    = {}", std::str::from_utf8(s).unwrap());
    println!("revcomp = {}", std::str::from_utf8(&unpack(rc, s.len())).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: reverse-complement on an unpacked sequence, for comparison.
    fn revcomp_naive(s: &[u8]) -> Vec<u8> {
        s.iter().rev().map(|&b| match b {
            b'A' => b'T', b'T' => b'A', b'C' => b'G', b'G' => b'C', _ => b
        }).collect()
    }

    #[test]
    fn reverse_pairs_acgt() {
        // ACGT -> TGCA (the bases are pair-reversed)
        let p = pack(b"ACGT");
        let r = reverse_pairs(p, 4);
        assert_eq!(unpack(r, 4), b"TGCA");
    }

    #[test]
    fn revcomp_palindrome() {
        // GAATTC is a palindrome (its revcomp is itself)
        let s = b"GAATTC";
        let rc = revcomp(pack(s), s.len());
        assert_eq!(unpack(rc, s.len()), s);
    }

    #[test]
    fn revcomp_acgt() {
        // ACGT revcomp is ACGT - yes, ACGT IS a palindrome
        let s = b"ACGT";
        let rc = revcomp(pack(s), s.len());
        assert_eq!(unpack(rc, s.len()), s);
    }

    #[test]
    fn revcomp_matches_naive_many() {
        for s in &[b"A" as &[u8], b"AG", b"ACGT", b"AACCGGTT", b"ACGTACGTACGTACGT"] {
            let rc_packed = unpack(revcomp(pack(s), s.len()), s.len());
            assert_eq!(rc_packed, revcomp_naive(s), "mismatch on {}", std::str::from_utf8(s).unwrap());
        }
    }

    #[test]
    fn revcomp_max_length() {
        let s = b"ACGTACGTACGTACGTACGTACGTACGTACGT"; // 32 bases
        let rc_packed = unpack(revcomp(pack(s), 32), 32);
        assert_eq!(rc_packed, revcomp_naive(s));
    }
}
