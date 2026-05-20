/// Return the greeting that the test expects.
///
/// The test asserts that this returns exactly `"Hello, ACGT!"`.
fn first_message() -> &'static str {
    // TODO: return the string "Hello, ACGT!"
    "todo"
}

fn main() {
    println!("{}", first_message());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greets_with_dna_bases() {
        assert_eq!(first_message(), "Hello, ACGT!");
    }
}
