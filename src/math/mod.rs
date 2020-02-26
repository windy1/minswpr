use rand::distributions::uniform::{SampleBorrow, SampleUniform};
use rand::Rng;
use std::collections::HashSet;
use std::hash::Hash;

mod point;

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

pub fn hex_to_rgb(hex: &str) -> (u8, u8, u8) {
    let hex = u32::from_str_radix(hex, 16).unwrap();
    (
        ((hex >> 16) & 0xff) as u8,
        ((hex >> 8) & 0xff) as u8,
        (hex & 0xff) as u8,
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_gen_rand_unique() {
        let res = super::gen_rand_unique(100, 0, 1000);
        assert_eq!(res.len(), 100);
    }
}
