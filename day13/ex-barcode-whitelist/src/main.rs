/// Pack a 16-bp DNA string into a u32 (16 bases x 2 bits = 32 bits).
/// Base at index 0 in bits 0..2, base at index i in bits (2*i)..(2*i+2).
/// Panics if s.len() != 16.
fn pack16(s: &[u8]) -> u32 {
    assert_eq!(s.len(), 16, "expected a 16-bp barcode, got {}", s.len());
    let mut acc = 0u32;
    for (i, &b) in s.iter().enumerate() {
        let v: u32 = match b { b'A' => 0, b'C' => 1, b'G' => 2, b'T' => 3, _ => 0 };
        acc |= v << (2 * i);
    }
    acc
}

/// Hamming distance between two packed 16-bp barcodes (range 0..=16).
fn hamming16(a: u32, b: u32) -> u32 {
    // TODO: same SWAR trick as exercise 3, on u32 instead of u64.
    //   let m = a ^ b;
    //   ((m | (m >> 1)) & 0x5555_5555).count_ones()
    let _ = (a, b);
    0
}

/// Search `whitelist` for the entry with the smallest Hamming distance to `query`.
/// Return `Some((best_index, best_distance))` if the smallest distance is <= `max_d`;
/// otherwise `None`.
///
/// Linear scan: O(whitelist.len()) per query. For real 6.7M whitelists this would be
/// too slow per read — production code uses a precomputed Hamming-1 index. Here we
/// only have a few thousand entries so linear scan is fine.
fn closest(query: u32, whitelist: &[u32], max_d: u32) -> Option<(usize, u32)> {
    // TODO: scan whitelist; keep the lowest hamming16 seen along with its index;
    //       return Some((idx, d)) if d <= max_d, else None.
    let _ = (query, whitelist, max_d);
    None
}

fn main() {
    // Toy whitelist with 4 entries.
    let whitelist: Vec<u32> = [
        b"AAAAAAAAAAAAAAAA" as &[u8],
        b"ACGTACGTACGTACGT",
        b"GGGGCCCCAAAATTTT",
        b"TTTTTTTTTTTTTTTT",
    ].iter().map(|s| pack16(s)).collect();

    // Pretend three reads showed up; one exact, one off by 1, one too far.
    let reads: &[&[u8]] = &[
        b"ACGTACGTACGTACGT", // exact match to whitelist[1]
        b"ACGTACGTACGTACGG", // last base T -> G: 1-Hamming match
        b"NNNNNNNNNNNNNNNN", // garbage (we treat N as A here for simplicity)
    ];

    for r in reads {
        let q = pack16(r);
        match closest(q, &whitelist, 2) {
            Some((i, d)) => println!("{} -> whitelist[{}] (Hamming {})",
                                     std::str::from_utf8(r).unwrap(), i, d),
            None         => println!("{} -> no whitelist match",
                                     std::str::from_utf8(r).unwrap()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn small_whitelist() -> Vec<u32> {
        [
            b"AAAAAAAAAAAAAAAA" as &[u8],
            b"ACGTACGTACGTACGT",
            b"GGGGCCCCAAAATTTT",
            b"TTTTTTTTTTTTTTTT",
        ].iter().map(|s| pack16(s)).collect()
    }

    #[test]
    fn exact_match() {
        let wl = small_whitelist();
        let q = pack16(b"ACGTACGTACGTACGT");
        assert_eq!(closest(q, &wl, 2), Some((1, 0)));
    }

    #[test]
    fn one_away() {
        let wl = small_whitelist();
        let q = pack16(b"ACGTACGTACGTACGG");  // last T -> G
        assert_eq!(closest(q, &wl, 2), Some((1, 1)));
    }

    #[test]
    fn too_far() {
        let wl = small_whitelist();
        let q = pack16(b"CCCCCCCCCCCCCCCC");
        // distance to AAA... = 16, to ACGT... = 12, to GGGG... = 12, to TTTT... = 16
        // min = 12; with max_d=2 should return None.
        assert_eq!(closest(q, &wl, 2), None);
    }

    #[test]
    fn ties_pick_first() {
        // Build two whitelist entries equidistant from the query
        let wl = vec![
            pack16(b"AAAAAAAAAAAAAAAA"),
            pack16(b"AAAAAAAAAAAAAAAC"),  // 1 away
            pack16(b"AAAAAAAAAAAAAACA"),  // also 1 away
        ];
        let q = pack16(b"AAAAAAAAAAAAAAAC");  // exact match to index 1
        assert_eq!(closest(q, &wl, 2), Some((1, 0)));
    }

    #[test]
    fn hamming_zero_for_identical() {
        let a = pack16(b"ACGTACGTACGTACGT");
        assert_eq!(hamming16(a, a), 0);
    }
}
