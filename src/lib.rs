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

        b.iter(|| {
            (0..10000).fold((), |_, _|
                encoder.encode("Numeric", "1", "H", "01234567")
            )
        });
    }
}