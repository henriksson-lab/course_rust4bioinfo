use std::env;

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
        b'N' => b'N',
        b'n' => b'n',
        other => panic!("complement_base: unsupported base {:?}", other as char),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 || args[1].len() != 1 {
        eprintln!("usage: complement <single base, e.g. A>");
        std::process::exit(1);
    }
    let base = args[1].as_bytes()[0];
    println!("{}", complement_base(base) as char);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complement_a() {
        assert_eq!(complement_base(b'A'), b'T');
    }

    #[test]
    fn complement_t() {
        assert_eq!(complement_base(b'T'), b'A');
    }

    #[test]
    fn complement_c() {
        assert_eq!(complement_base(b'C'), b'G');
    }

    #[test]
    fn complement_g() {
        assert_eq!(complement_base(b'G'), b'C');
    }

    #[test]
    fn complement_lowercase_g() {
        assert_eq!(complement_base(b'g'), b'c');
    }

    #[test]
    fn complement_n_passthrough() {
        assert_eq!(complement_base(b'N'), b'N');
    }

    #[test]
    fn complement_lowercase_n() {
        assert_eq!(complement_base(b'n'), b'n');
    }

    #[test]
    #[should_panic]
    fn complement_invalid_panics() {
        let _ = complement_base(b'X');
    }
}
