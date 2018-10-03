#![feature(non_ascii_idents)]
#![feature(test)]
extern crate test;

mod encoder;

#[cfg(test)]
mod tests {
    use super::*;
    use self::test::Bencher;

    #[bench]
    fn encoder_test(b: &mut Bencher) {
        use self::encoder::Encoder;
        let encoder = Encoder::new();

        // Numeric
//        b.iter(|| {
//            (0..1).fold((), |_, _|
//                encoder.encode("Numeric", "1", "H", "01234567")
//            )
//        });

        // Alphanumeric
        b.iter(|| (0..1).fold((), |_, _| encoder.encode("Alphanumeric", "1", "H", "XAVIER")));
    }

    fn check_format(mut fmt: usize, g: usize) -> usize {
        for i in (0..5).rev() {
            if fmt & (1 << (i + 10)) != 0 {
                fmt ^= g << i;
            }
        }

        fmt
    }

    #[test]
    fn encoded_format() {
        let mut fmt = 0b000111101011001;
        fmt = (fmt << 10) ^ check_format(fmt << 10, 0b10100110111);

        println!("{}", fmt);
        println!("{}", check_format(25722, 1335));
    }

    fn hamming_distance(mut x: usize, y: usize) -> usize {
        let mut d = 0;
        x ^= y;

        while x != 0 {
            d += x & 1;
            x >>= 1;
        }

        d
    }

    #[test]
    fn decode_format() { println!("{}", hamming_distance(0b101100, 0b010011)); }

    static mut GF_EXP: &mut [usize; 512] = &mut [0; 512];
    static mut GF_LOG: &mut [usize; 256] = &mut [0; 256];

    unsafe fn init_tables(prim: Option<usize>) {
        let prim = if let Some(prim) = prim { prim } else { 0x11d };

        let mut x = 1;

        for i in 0..255 {
            GF_EXP[i] = x;
            GF_LOG[x] = i;

            x <<= 1;

            if x & 0x100 != 0 { x ^= prim; }
        }

        for i in 255..512 { GF_EXP[i] = GF_EXP[i - 255]; }

        println!("{:?}\n{:?}", GF_EXP.to_vec(), GF_LOG.to_vec());
    }

    #[test]
    fn print_tables() {
        unsafe {
            init_tables(None);
        }
    }
}
