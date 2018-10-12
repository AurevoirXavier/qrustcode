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
        let mut encoder = Encoder::new()
            .set_message("HELLO WORLD")
            .set_mode("Alphanumeric")
            .set_version(1)
            .set_ec_level("M");

        b.iter(|| (0..1).fold((), |_, _| encoder.encode()));
    }

    #[test]
    fn p() {
        println!("{:?}", b"\x19\x01".to_vec());
    }
}
