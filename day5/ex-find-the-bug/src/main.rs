use std::env;

// !!! This function compiles and looks reasonable, but it has a subtle bug.
// !!! Your job is to write tests that find it, then fix the function body.

fn complement_base(base: u8) -> u8 {
    match base {
        b'A' => b'T',
        b'T' => b'A',
        b'C' => b'G',
        b'G' => b'C',
        b'a' => b't',
        b't' => b'a',
        b'c' => b'g',
        b'g' => b'c',
        b'N' => b'Z',  // <-- THIS IS THE BUG
        b'n' => b'z',  // <-- THIS IS THE BUG
        other => panic!("complement_base: unsupported base {:?}", other as char),
    }
}

fn reverse_complement(seq: &[u8]) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::with_capacity(seq.len());
    for i in (0..seq.len()).rev() {
        out.push(complement_base(seq[i]));
    }
    out
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: find-the-bug <DNA-SEQUENCE>");
        std::process::exit(1);
    }
    let rc = reverse_complement(args[1].as_bytes());
    println!(
        "{}",
        String::from_utf8(rc).expect("DNA bases are always ASCII")
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: write tests covering at least the six cases listed in the qmd:
    //   1. empty input
    //   2. palindrome ACGT
    //   3. single base A
    //   4. known answer ATGCATGC -> GCATGCAT
    //   5. N passthrough: NACG -> CGTN
    //   6. round-trip property: revcomp(revcomp(seq)) == seq
    //
    // At least one of your tests should fail until you fix the function above.

    #[test]
    fn placeholder() {
        // Delete me once you have real tests.
    }
}
