#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use crate::*;

    use super::*;
    use test::Bencher;

    #[bench]
    fn init_sled(b: &mut Bencher) {
        b.iter(|| initialize_sled());
    }

    #[test]
    fn init_sled_size() {
        println!("{}", measure_sled_size())
    }

}
