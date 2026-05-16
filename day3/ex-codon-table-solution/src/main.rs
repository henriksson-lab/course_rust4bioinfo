use std::env;

fn translate_codon(codon: [u8; 3]) -> Option<u8> {
    let upper = [
        codon[0].to_ascii_uppercase(),
        codon[1].to_ascii_uppercase(),
        codon[2].to_ascii_uppercase(),
    ];
    match &upper {
        b"TTT" | b"TTC" => Some(b'F'),
        b"TTA" | b"TTG" | b"CTT" | b"CTC" | b"CTA" | b"CTG" => Some(b'L'),
        b"ATT" | b"ATC" | b"ATA" => Some(b'I'),
        b"ATG" => Some(b'M'),
        b"GTT" | b"GTC" | b"GTA" | b"GTG" => Some(b'V'),
        b"TCT" | b"TCC" | b"TCA" | b"TCG" | b"AGT" | b"AGC" => Some(b'S'),
        b"CCT" | b"CCC" | b"CCA" | b"CCG" => Some(b'P'),
        b"ACT" | b"ACC" | b"ACA" | b"ACG" => Some(b'T'),
        b"GCT" | b"GCC" | b"GCA" | b"GCG" => Some(b'A'),
        b"TAT" | b"TAC" => Some(b'Y'),
        b"TAA" | b"TAG" | b"TGA" => Some(b'*'),
        b"CAT" | b"CAC" => Some(b'H'),
        b"CAA" | b"CAG" => Some(b'Q'),
        b"AAT" | b"AAC" => Some(b'N'),
        b"AAA" | b"AAG" => Some(b'K'),
        b"GAT" | b"GAC" => Some(b'D'),
        b"GAA" | b"GAG" => Some(b'E'),
        b"TGT" | b"TGC" => Some(b'C'),
        b"TGG" => Some(b'W'),
        b"CGT" | b"CGC" | b"CGA" | b"CGG" | b"AGA" | b"AGG" => Some(b'R'),
        b"GGT" | b"GGC" | b"GGA" | b"GGG" => Some(b'G'),
        _ => None,
    }
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
        assert_eq!(translate_codon(*b"RTG"), None);
        assert_eq!(translate_codon(*b"YAA"), None);
    }

    #[test]
    fn empty_garbage_returns_none() {
        assert_eq!(translate_codon([0, 0, 0]), None);
    }
}
