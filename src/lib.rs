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

    #[test]
    fn check_format() {
        let mut fmt = 0b000111101011001;
        let g = 0b10100110111;

        for i in (0..5).rev() {
            if fmt & (1 << (i + 10)) !=0 {
                fmt ^= g << i;
            }
        }

        println!("{}", fmt);
    }
}