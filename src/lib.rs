#![feature(test)]

extern crate encoding_rs;
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
//            .mode("Kanji")
//            .version(2)
            .ec_level("M");

        b.iter(|| (0..1).fold((), |_, _| encoder.encode("ハロー・ワールド")));
    }

    #[test]
    fn test() {
        use encoding_rs::SHIFT_JIS;
        let (c, _, _) = SHIFT_JIS.encode("茗");
        println!("{:?}", c.to_vec());
    }
}
