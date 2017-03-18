#![feature(test)]

extern crate test;

/// left-to-right binary exponentiation scheme
pub mod left_to_right {

    use std::f64;

    /// modular exponentiation `(16 ^ exponent) % modulus`
    pub fn modular_pow(mut exponent: u64, modulus: f64) -> f64 {
        if modulus == 1.0 { return 0.0 }
        assert!((modulus - 1.0) * (modulus - 1.0) < f64::MAX);
        let mut result = 1.0;
        let mut base = 16.0 % modulus;
        while exponent > 0 {
            if exponent % 2 == 1 { // mod can be a simple mask !
                result = (result * base) % modulus;
            }
            exponent >>= 1;
            base = (base * base) % modulus;
        }
        result
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use test::Bencher;

        #[test]
        fn test_modular_pow() {
            // (16^13) % 497
            assert_eq!(modular_pow(13, 497.0), 219.0)
        }

        // #[bench]
        // fn bench_add_two(b: &mut Bencher) {
        //     b.iter(|| add_two(2));
        // }
    }
}

/// right-to-left binary exponentiation scheme
pub mod right_to_left {

    /// modular exponentiation `(16 ^ exponent) % modulus`
    pub fn modular_pow(mut exponent: f64, modulus: f64) -> f64 { // FIXME: exponent can be u64
        if modulus == 1.0 { return 0.0 }

        // move this elsewhere
        // fill the power of two table
        let exponents_pow2 = { // FIXME: compile-time this
            let mut exponents_pow2 = [1.0; 25];
            for i in 1..exponents_pow2.len() { // better slice::windows_mut(2)
                exponents_pow2[i] = 2.0 * exponents_pow2[i - 1];
            }
            exponents_pow2
        };

        println!("exponents_pow2: {:?}", exponents_pow2);

        // Find the greatest power of two less than or equal to exponent
        let (i, mut exponent_pow2) = {
            exponents_pow2.iter()
                          .cloned()
                          .enumerate()
                          .take_while(|&(_, x)| x <= exponent)
                          .last()
                          .unwrap_or_else(|| {
                              let last_id = exponents_pow2.len() - 1;
                              (last_id, exponents_pow2[last_id])
                          })
        };

        println!("exponent: {:?}", exponent);
        println!("exponent_pow2: {:?}", exponent_pow2);

        // Perform binary exponentiation algorithm modulo modulus.
        let mut result = 1.0;
        for _ in 1..i + 1 {
            if exponent >= exponent_pow2 {
                result *= 16.0;
                result = result - (result / modulus).trunc() * modulus;
                exponent -= exponent_pow2;
            }
            exponent_pow2 *= 0.5;
            if exponent_pow2 >= 1.0 {
                result *= result;
                result = result - (result / modulus).trunc() * modulus;
                // TODO: primitive.f64.html#method.mul_add
            }
        }
        result
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use test::Bencher;

        #[test]
        fn test_modular_pow() {
            // (16^13) % 497
            assert_eq!(modular_pow(13.0, 497.0), 219.0)
        }

        // #[bench]
        // fn bench_add_two(b: &mut Bencher) {
        //     b.iter(|| add_two(2));
        // }
    }
}
