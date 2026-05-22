/// Encode a single DNA base into 2 bits.
/// A=0b00, C=0b01, G=0b10, T=0b11. Panics on any other character.
fn encode_base(b: u8) -> u64 {
    match b {
        b'A' => 0,
        b'C' => 1,
        b'G' => 2,
        b'T' => 3,
        _ => panic!("non-ACGT base {}", b as char),
    }
}

/// Decode a 2-bit value back to a DNA base (0->A, 1->C, 2->G, 3->T).
fn decode_base(v: u64) -> u8 {
    b"ACGT"[v as usize]
}

/// Pack a DNA string (length 1..=32, chars ACGT) into a single u64.
/// The base at index 0 of the string lives in bits 0..2 of the result;
/// the base at index i lives in bits (2*i)..(2*i+2).
fn pack(s: &[u8]) -> u64 {
    let mut acc = 0u64;
    for (i, &b) in s.iter().enumerate() {
        acc |= encode_base(b) << (2 * i);
    }
    acc
}

/// Unpack a u64 back to a DNA string of the given length.
fn unpack(packed: u64, len: usize) -> Vec<u8> {
    (0..len)
        .map(|i| decode_base((packed >> (2 * i)) & 0b11))
        .collect()
}

/// Return the base at index i of a packed sequence.
fn nth_base(packed: u64, i: usize) -> u8 {
    decode_base((packed >> (2 * i)) & 0b11)
}

fn main() {
    let s = b"ACGTACGT";
    let p = pack(s);
    println!("packed = 0x{:016x}", p);
    println!("unpacked = {}", std::str::from_utf8(&unpack(p, s.len())).unwrap());
    println!("nth_base(p, 2) = {}", nth_base(p, 2) as char);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode_roundtrip() {
        for &b in b"ACGT" {
            assert_eq!(decode_base(encode_base(b)), b);
        }
    }

    #[test]
    fn pack_single_a() {
        // "A" -> 0b00 -> 0
        assert_eq!(pack(b"A"), 0);
    }

    #[test]
    fn pack_acgt() {
        // ACGT, index 0..3:
        //   A=00 in bits 0..2
        //   C=01 in bits 2..4
        //   G=10 in bits 4..6
        //   T=11 in bits 6..8
        // So overall bits: 11 10 01 00 = 0xe4 = 228
        assert_eq!(pack(b"ACGT"), 0b11_10_01_00);
    }

    #[test]
    fn roundtrip_string() {
        let s = b"ACGTAACCGGTT";
        assert_eq!(unpack(pack(s), s.len()), s);
    }

    #[test]
    fn nth_base_matches_string() {
        let s = b"ACGTAACCGGTT";
        let p = pack(s);
        for i in 0..s.len() {
            assert_eq!(nth_base(p, i), s[i]);
        }
    }

    #[test]
    fn pack_max_length() {
        let s = b"ACGTACGTACGTACGTACGTACGTACGTACGT"; // 32 bases
        assert_eq!(unpack(pack(s), 32), s);
    }
}
