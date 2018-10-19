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
//            .mode("Alphanumeric")
//            .version(1)
            .ec_level("M");

        b.iter(|| (0..1).fold((), |_, _| encoder.encode("一二")));
    }

    #[test]
    fn test() {
        println!("{}", '\u{6211}' < '\u{0800}' );
    }
}
