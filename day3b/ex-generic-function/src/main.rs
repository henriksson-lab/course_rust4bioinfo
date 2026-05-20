/// Count how many elements of `items` equal `target`.
///
/// Works for ANY type that implements `PartialEq` — bytes, strings,
/// enums, your own structs.
fn count_matching<T: PartialEq>(items: &[T], target: &T) -> usize {
    // TODO: filter the slice for items equal to target, then count
    // Hint: items.iter().filter(|&item| item == target).count()
    let _ = (items, target);
    0
}

#[derive(PartialEq)]
enum Base { A, C, G, T }

fn main() {
    let bytes = b"ACGTACGT";
    println!("As in ACGTACGT: {}", count_matching(bytes, &b'A'));

    let words = vec!["chr1".to_string(), "chr2".to_string(), "chr1".to_string()];
    println!("chr1s in [chr1, chr2, chr1]: {}", count_matching(&words, &"chr1".to_string()));

    let bases = vec![Base::A, Base::C, Base::G, Base::A];
    println!("As in bases: {}", count_matching(&bases, &Base::A));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_a_in_acgt() {
        assert_eq!(count_matching(b"ACGTACGT", &b'A'), 2);
    }

    #[test]
    fn count_g_in_acgt() {
        assert_eq!(count_matching(b"ACGTACGT", &b'G'), 2);
    }

    #[test]
    fn count_missing() {
        assert_eq!(count_matching(b"ACGT", &b'N'), 0);
    }

    #[test]
    fn count_strings() {
        let v = vec!["chr1".to_string(), "chr2".to_string(), "chr1".to_string()];
        assert_eq!(count_matching(&v, &"chr1".to_string()), 2);
    }

    #[test]
    fn count_empty_slice() {
        let v: Vec<u8> = Vec::new();
        assert_eq!(count_matching(&v, &b'A'), 0);
    }

    #[test]
    fn count_custom_enum() {
        let v = vec![Base::A, Base::C, Base::G, Base::A, Base::A];
        assert_eq!(count_matching(&v, &Base::A), 3);
    }
}
