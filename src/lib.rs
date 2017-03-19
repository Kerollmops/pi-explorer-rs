#![feature(test)]

extern crate test;

use std::f64;

/// Left-to-right modular exponentiation, `(16 ^ exponent) % modulus`.
pub fn modular_pow(mut exponent: u64, modulus: f64) -> f64 {
    if modulus == 1.0 { return 0.0 }
    assert!((modulus - 1.0) * (modulus - 1.0) < f64::MAX);
    let mut result = 1.0;
    let mut base = 16.0 % modulus;
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base) % modulus;
        }
        exponent >>= 1;
        base = (base * base) % modulus;
    }
    result
}

/// This routine evaluates the series  sum_k 16^(id-k)/(8*k+m)
/// using the modular exponentiation technique.
pub fn series(m: f64, id: i32) -> f64 {
    let mut series = 0.0;

    // Sum the series up to id
    for k in 0..id {
        let ak = 8.0_f64.mul_add(k as f64, m);
        let t = modular_pow((id - k) as u64, ak);
        // TODO: reduce to one line ?
        series += t / ak;
        series -= series.trunc();
    }

    // Compute a few terms where k >= id
    for k in id..id + 100 + 1 { // FIXME: inclusive range
        let ak = 8.0_f64.mul_add(k as f64, m);
        let t = 16.0_f64.powi(id - k) / ak; // TODO: improve this ?
        if t < f64::EPSILON { break }
        // TODO: reduce to one line ?
        series += t;
        series -= series.trunc();
    }
    series
}

pub fn get_digit(id: i32) -> u8 {
    let s1 = series(1.0, id);
    let s2 = series(4.0, id);
    let s3 = series(5.0, id);
    let s4 = series(6.0, id);

    // FIXME: use simd
    let mut pid = (4.0 * s1) - (2.0 * s2) - s3 - s4;
    pid = pid - pid.trunc() + 1.0; // FIXME: change this (ugly, not efficient)

    // FIXME: not efficient
    let mut y = pid.abs();
    y = 16.0 * (y - y.floor());

    // FIXME: not efficient
    let first = y as u8;
    y = 16.0 * (y - y.floor());
    let second = y as u8;

    (first << 4) | second
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_some_digits() {
        assert_eq!(get_digit(0), 0x24);
        assert_eq!(get_digit(1), 0x43);
        assert_eq!(get_digit(2), 0x3f);
        assert_eq!(get_digit(3), 0xf6);
        assert_eq!(get_digit(4), 0x6a);
    }

    #[test]
    fn test_a_far_digit() {
        assert_eq!(get_digit(100_000), 0x35);
    }

    #[test]
    fn test_modular_pow() {
        // (16^13) % 497 = 219
        assert_eq!(modular_pow(13, 497.0), 219.0)
    }

    #[bench]
    fn bench_modular_pow(b: &mut Bencher) {
        b.iter(|| modular_pow(1_300_000_000_000, 497_323_987_988_000.0))
    }
}
