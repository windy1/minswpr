mod point;

use rand::distributions::uniform::{SampleBorrow, SampleUniform};
use rand::Rng;
use std::collections::HashSet;
use std::hash::Hash;

pub use self::point::*;

pub fn gen_rand_unique<T, B>(n: usize, lo: B, hi: B) -> Vec<T>
where
    T: SampleUniform + Eq + Hash,
    B: SampleBorrow<T> + Sized + Copy,
{
    let mut rng = rand::thread_rng();
    let mut res = HashSet::with_capacity(n);
    while res.len() < n {
        let v = rng.gen_range(lo, hi);
        if !res.contains(&v) {
            res.insert(v);
        }
    }
    res.drain().collect()
}

pub fn hex_to_rgb(hex: &str) -> Result<(u8, u8, u8), String> {
    let hex = u32::from_str_radix(hex, 16).map_err(|e| e.to_string())?;
    Ok((
        ((hex >> 16) & 0xff) as u8,
        ((hex >> 8) & 0xff) as u8,
        (hex & 0xff) as u8,
    ))
}

#[cfg(test)]
mod tests {
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
    fn test_hex_to_rgb() -> Result<(), String> {
        assert_eq!((255, 0, 0), super::hex_to_rgb("ff0000")?);
        assert_eq!((0, 255, 0), super::hex_to_rgb("00ff00")?);
        assert_eq!((0, 0, 255), super::hex_to_rgb("0000ff")?);
        assert_eq!((0, 0, 0), super::hex_to_rgb("000000")?);
        assert_eq!((255, 255, 255), super::hex_to_rgb("ffffff")?);
        assert_eq!((255, 0, 255), super::hex_to_rgb("ff00ff")?);
        Ok(())
    }
}
