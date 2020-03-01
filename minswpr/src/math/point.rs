#![macro_use]

use serde::Deserialize;
use std::convert::TryInto;
use std::iter::{Iterator, Sum};
use std::num::TryFromIntError;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

macro_rules! point {
    ($x:expr, $y:expr) => {
        Point::new($x, $y)
    };
}

pub type RawPoint<T = i32> = (T, T);

#[derive(new, Debug, Eq, PartialEq, Copy, Clone, Hash, Deserialize, Default)]
pub struct Point<T = i32>
where
    T: Copy,
{
    pub x: T,
    pub y: T,
}

impl Point<u32> {
    pub fn as_i32(self) -> Point {
        self.into()
    }
}

impl Point {
    pub fn try_as_u32(self) -> Result<Point<u32>, TryFromIntError> {
        self.try_into()
    }
}

impl Into<Point> for Point<u32> {
    fn into(self) -> Point {
        point!(self.x as i32, self.y as i32)
    }
}

impl Into<RawPoint> for Point<u32> {
    fn into(self) -> RawPoint {
        (self.into(): Point).into()
    }
}

impl TryInto<Point<u32>> for Point<i32> {
    type Error = TryFromIntError;

    fn try_into(self) -> Result<Point<u32>, Self::Error> {
        Ok(point!(self.x.try_into()?, self.y.try_into()?))
    }
}

impl TryInto<RawPoint<u32>> for Point<i32> {
    type Error = TryFromIntError;

    fn try_into(self) -> Result<RawPoint<u32>, Self::Error> {
        Ok((self.try_into()?: Point<u32>).into())
    }
}

impl<T: Copy> Into<RawPoint<T>> for Point<T> {
    fn into(self) -> RawPoint<T> {
        (self.x, self.y)
    }
}

impl<T: Copy> From<RawPoint<T>> for Point<T> {
    fn from(p: RawPoint<T>) -> Self {
        point!(p.0, p.1)
    }
}

impl<T> Sum for Point<T>
where
    T: Sum<T> + Copy + Default + Add<Output = T>,
{
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(
            Point {
                x: T::default(),
                y: T::default(),
            },
            |a, b| Point {
                x: a.x + b.x,
                y: a.y + b.y,
            },
        )
    }
}

impl<T> Add for Point<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T> Add<RawPoint<T>> for Point<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, rhs: RawPoint<T>) -> Self::Output {
        self.add(Self::new(rhs.0, rhs.1))
    }
}

impl<T> AddAssign for Point<T>
where
    T: AddAssign + Copy,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> AddAssign<RawPoint<T>> for Point<T>
where
    T: AddAssign + Copy,
{
    fn add_assign(&mut self, rhs: RawPoint<T>) {
        self.add_assign(Self::new(rhs.0, rhs.1))
    }
}

impl<T> Sub for Point<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T> Sub<RawPoint<T>> for Point<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Self;

    fn sub(self, rhs: RawPoint<T>) -> Self::Output {
        self.sub(Self::new(rhs.0, rhs.1))
    }
}

impl<T> SubAssign for Point<T>
where
    T: SubAssign + Copy,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T> SubAssign<RawPoint<T>> for Point<T>
where
    T: SubAssign + Copy,
{
    fn sub_assign(&mut self, rhs: RawPoint<T>) {
        self.sub_assign(Self::new(rhs.0, rhs.1))
    }
}

impl<T> Mul for Point<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl<T> Mul<RawPoint<T>> for Point<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: RawPoint<T>) -> Self::Output {
        self.mul(Self::new(rhs.0, rhs.1))
    }
}

impl<T> MulAssign for Point<T>
where
    T: MulAssign + Copy,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl<T> MulAssign<RawPoint<T>> for Point<T>
where
    T: MulAssign + Copy,
{
    fn mul_assign(&mut self, rhs: RawPoint<T>) {
        self.mul_assign(Self::new(rhs.0, rhs.1))
    }
}

impl<T> Div for Point<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.x / rhs.x, self.y / rhs.y)
    }
}

impl<T> Div<RawPoint<T>> for Point<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Self;

    fn div(self, rhs: RawPoint<T>) -> Self::Output {
        self.div(Self::new(rhs.0, rhs.1))
    }
}

impl<T> DivAssign for Point<T>
where
    T: DivAssign + Copy,
{
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl<T> DivAssign<RawPoint<T>> for Point<T>
where
    T: DivAssign + Copy,
{
    fn div_assign(&mut self, rhs: RawPoint<T>) {
        self.div_assign(Self::new(rhs.0, rhs.1))
    }
}

pub type Dimen<T = u32> = Point<T>;

impl<T> Dimen<T>
where
    T: Copy,
{
    pub fn width(&self) -> T {
        self.x
    }

    pub fn height(&self) -> T {
        self.y
    }

    pub fn set_width(&mut self, width: T) {
        self.x = width
    }

    pub fn set_height(&mut self, height: T) {
        self.y = height
    }
}

#[cfg(test)]
mod tests {
    use super::Point;

    #[test]
    fn test_point_ops() {
        let p1 = point!(0, 0);
        let p2 = point!(1, 2);
        let p3 = point!(3, 4);
        let p4 = point!(4, 6);
        let r2 = (1, 2);
        let r3 = (3, 4);
        assert_eq!(p2, p1 + p2);
        assert_eq!(p4, p2 + p3);
        assert_eq!(p2, p1 + r2);
        assert_eq!(p4, p2 + r3);
    }

    #[test]
    fn test_point_sum() {
        let p = vec![point!(1, 2), point!(3, 4), point!(5, 6)];
        assert_eq!(point!(9, 12), p.iter().cloned().sum());
    }
}
