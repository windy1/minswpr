mod point;

pub use self::point::*;

use rand::distributions::uniform::{SampleBorrow, SampleUniform};
use rand::Rng;
use std::collections::HashSet;
use std::hash::Hash;
use std::num::ParseIntError;

/// Returns a uniform random distribution of unique values of `n` elements in
/// the range `(lo, hi)`
pub fn gen_rand_unique<T, B>(n: usize, lo: B, hi: B) -> Vec<T>
where
    T: SampleUniform + Eq + Hash,
    B: SampleBorrow<T> + Copy,
{
    let mut rng = rand::thread_rng();
    let mut res = HashSet::with_capacity(n);
    while res.len() < n {
        let v = rng.gen_range(lo, hi);
        res.insert(v);
    }
    res.drain().collect()
}

/// Returns an RGB tuple from the specified `hex` string
pub fn hex_to_rgb(hex: &str) -> Result<(u8, u8, u8), ParseIntError> {
    let hex = u32::from_str_radix(hex, 16)?;
    Ok((
        ((hex >> 16) & 0xff) as u8,
        ((hex >> 8) & 0xff) as u8,
        (hex & 0xff) as u8,
    ))
}

#[cfg(test)]
mod tests {
    use std::num::ParseIntError;

    #[test]
    fn test_gen_rand_unique() {
        let res = super::gen_rand_unique(100, 0, 1000);
        assert_eq!(res.len(), 100);
    }

    #[test]
    #[should_panic]
    fn test_hex_to_rgb_empty_str() {
        super::hex_to_rgb("").unwrap();
    }

    #[test]
    fn test_hex_to_rgb() -> Result<(), ParseIntError> {
        assert_eq!((255, 0, 0), super::hex_to_rgb("ff0000")?);
        assert_eq!((0, 255, 0), super::hex_to_rgb("00ff00")?);
        assert_eq!((0, 0, 255), super::hex_to_rgb("0000ff")?);
        assert_eq!((0, 0, 0), super::hex_to_rgb("000000")?);
        assert_eq!((255, 255, 255), super::hex_to_rgb("ffffff")?);
        assert_eq!((255, 0, 255), super::hex_to_rgb("ff00ff")?);
        Ok(())
    }
}
