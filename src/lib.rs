/// left-to-right binary exponentiation scheme
pub mod left_to_right {

    /// modular exponentiation `(16 ^ exponent) % modulus`
    pub fn modular_pow(mut exponent: u64, modulus: f64) -> f64 {
        if modulus == 1.0 { return 0.0 }
        //assert!((modulus - 1) * (modulus - 1) does not overflow base);
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
}

/// right-to-left binary exponentiation scheme
pub mod right_to_left {

    /// modular exponentiation `(16 ^ exponent) % modulus`
    pub fn modular_pow(mut exponent: f64, modulus: f64) -> f64 {
        if modulus == 1.0 { return 0.0 }

        // move this elsewhere
        // fill the power of two table
        let exponents_pow2 = { // FIXME: compile-time this
            let mut exponents_pow2 = [1.0; 25];
            for i in 1..exponents_pow2.len() - 1 { // better slice::windows_mut(2)
                exponents_pow2[i] = 2.0 * exponents_pow2[i - 1];
            }
            exponents_pow2
        };

        // Find the greatest power of two less than or equal to exponent
        let (i, mut exponent_pow2) = {
            exponents_pow2.iter().cloned()
                                 .enumerate()
                                 .take_while(|&(_, x)| x > exponent)
                                 .next().unwrap()
        };

        // Perform binary exponentiation algorithm modulo modulus.
        let mut result = 1.0;
        for _ in 1..i {
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
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
