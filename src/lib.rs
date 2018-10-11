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
        b.iter(|| {
            (0..1).fold((), |_, _|
                encoder.encode("Numeric", "1", "H", "01234567")
            )
        });

        // Alphanumeric
        b.iter(|| (0..1).fold((), |_, _| encoder.encode("Alphanumeric", "1", "M", "HELLO WORLD")));
    }

    #[test]
    fn p() {
        println!("{}", b':');
    }
}
