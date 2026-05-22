/// Format a u8 in three bases: decimal, binary (8 digits, zero-padded), hex (2 digits).
/// Example: 13 -> "13 = 0b00001101 = 0x0d"
fn show_three_bases(x: u8) -> String {
    // TODO: use format!("{} = 0b{:08b} = 0x{:02x}", x, x, x)
    let _ = x;
    String::new()
}

/// Return the number of 1-bits in x.
fn bit_count(x: u32) -> u32 {
    // TODO: use the standard library: x.count_ones()
    let _ = x;
    0
}

/// Return (a & b, a | b, a ^ b).
fn three_ops(a: u8, b: u8) -> (u8, u8, u8) {
    // TODO: straightforward
    let _ = (a, b);
    (0, 0, 0)
}

/// Set bit `i` (counting from the least-significant) in x and return the new value.
/// Bits beyond i=7 should be a no-op (the caller is responsible; tests only pass 0..8).
fn set_bit(x: u8, i: u8) -> u8 {
    // TODO: x | (1 << i)
    let _ = (x, i);
    0
}

/// Clear bit `i` in x and return the new value.
fn clear_bit(x: u8, i: u8) -> u8 {
    // TODO: x & !(1 << i)
    let _ = (x, i);
    0
}

fn main() {
    for n in [0u8, 1, 2, 5, 13, 42, 128, 255] {
        println!("{}", show_three_bases(n));
    }
    let (a, b) = (0b1100u8, 0b1010u8);
    let (and, or, xor) = three_ops(a, b);
    println!("{:04b} & {:04b} = {:04b}", a, b, and);
    println!("{:04b} | {:04b} = {:04b}", a, b, or);
    println!("{:04b} ^ {:04b} = {:04b}", a, b, xor);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn show_thirteen() {
        assert_eq!(show_three_bases(13), "13 = 0b00001101 = 0x0d");
    }

    #[test]
    fn show_zero() {
        assert_eq!(show_three_bases(0), "0 = 0b00000000 = 0x00");
    }

    #[test]
    fn show_max() {
        assert_eq!(show_three_bases(255), "255 = 0b11111111 = 0xff");
    }

    #[test]
    fn count_bits() {
        assert_eq!(bit_count(0), 0);
        assert_eq!(bit_count(1), 1);
        assert_eq!(bit_count(0b1011_0011), 5);
        assert_eq!(bit_count(u32::MAX), 32);
    }

    #[test]
    fn ops() {
        assert_eq!(three_ops(0b1100, 0b1010), (0b1000, 0b1110, 0b0110));
        assert_eq!(three_ops(0xff, 0x00), (0, 0xff, 0xff));
    }

    #[test]
    fn set_clear() {
        assert_eq!(set_bit(0, 0), 1);
        assert_eq!(set_bit(0, 7), 0b1000_0000);
        assert_eq!(set_bit(0b1000_0001, 4), 0b1001_0001);
        assert_eq!(clear_bit(0xff, 0), 0b1111_1110);
        assert_eq!(clear_bit(0b1001_0001, 4), 0b1000_0001);
        assert_eq!(clear_bit(0, 3), 0); // already clear
    }
}
