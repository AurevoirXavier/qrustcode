#![feature(test)]

extern crate encoding_rs;
//extern crate image;
#[macro_use]
extern crate lazy_static;

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

        b.iter(|| (0..1).fold((), |_, _| {
            encoder
                .encode("ハロー・ワールド")
                .as_matrix();
        }));
    }

    #[test]
    fn test() {
        use self::encoder::Encoder;

        let mut encoder = Encoder::new()
            .ec_level("M")
            .encode("ハロー・ワールド")
            .as_matrix();

//        println!("{:?}", );
    }
}
