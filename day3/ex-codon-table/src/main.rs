use std::env;

/// Translate one DNA codon to its amino acid (single-letter code).
///
/// Returns Some(b'*') for stop codons (TAA, TAG, TGA), Some(amino acid)
/// for the other 61 sense codons, and None for any codon containing a
/// non-ACGT byte (typically N or IUPAC ambiguity codes).
///
/// The function is case-insensitive on the input.
fn translate_codon(codon: [u8; 3]) -> Option<u8> {
    // TODO:
    //   1. uppercase each byte with u8::to_ascii_uppercase
    //   2. match the resulting [u8; 3] against the 61 sense codons
    //      and the 3 stop codons; everything else is None
    //   See the qmd for the complete codon table.
    let _ = codon;
    None
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 || args[1].len() != 3 {
        eprintln!("usage: codon-table <THREE-LETTER-CODON>");
        std::process::exit(1);
    }
    let bytes = args[1].as_bytes();
    let codon = [bytes[0], bytes[1], bytes[2]];
    match translate_codon(codon) {
        Some(aa) => println!("{}", aa as char),
        None => println!("None"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_codon() {
        assert_eq!(translate_codon(*b"ATG"), Some(b'M'));
    }

    #[test]
    fn stop_codons() {
        assert_eq!(translate_codon(*b"TAA"), Some(b'*'));
        assert_eq!(translate_codon(*b"TAG"), Some(b'*'));
        assert_eq!(translate_codon(*b"TGA"), Some(b'*'));
    }

    #[test]
    fn tryptophan_only_one() {
        // W has just one codon: TGG. TGA is stop, not W.
        assert_eq!(translate_codon(*b"TGG"), Some(b'W'));
    }

    #[test]
    fn case_insensitive() {
        assert_eq!(translate_codon(*b"atg"), Some(b'M'));
        assert_eq!(translate_codon(*b"AtG"), Some(b'M'));
    }

    #[test]
    fn alanine_all_four() {
        for codon in [*b"GCT", *b"GCC", *b"GCA", *b"GCG"] {
            assert_eq!(translate_codon(codon), Some(b'A'), "codon {:?}", codon);
        }
    }

    #[test]
    fn leucine_all_six() {
        for codon in [*b"TTA", *b"TTG", *b"CTT", *b"CTC", *b"CTA", *b"CTG"] {
            assert_eq!(translate_codon(codon), Some(b'L'), "codon {:?}", codon);
        }
    }

    #[test]
    fn n_returns_none() {
        assert_eq!(translate_codon(*b"ATN"), None);
        assert_eq!(translate_codon(*b"NTG"), None);
        assert_eq!(translate_codon(*b"NNN"), None);
    }

    #[test]
    fn iupac_ambiguity_returns_none() {
        // R = A or G, Y = C or T, etc. — not in the standard table.
        assert_eq!(translate_codon(*b"RTG"), None);
        assert_eq!(translate_codon(*b"YAA"), None);
    }

    #[test]
    fn empty_garbage_returns_none() {
        assert_eq!(translate_codon([0, 0, 0]), None);
    }
}
