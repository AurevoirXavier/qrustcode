mod encoder;

#[cfg(test)]
mod tests {
    use super::*;

    # [test]
    fn encoder_test() {
        use self::encoder::Encoder;
        let encoder = Encoder::new();
    }
}