use std::ops;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub struct Point<T: Copy = i32> {
    pub x: T,
    pub y: T,
}

pub type RawPoint<T = i32> = (T, T);

impl<T: Copy> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: ops::Add<Output = T> + Copy> ops::Add for Point<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: ops::Add<Output = T> + Copy> ops::Add<RawPoint<T>> for Point<T> {
    type Output = Self;

    fn add(self, rhs: RawPoint<T>) -> Self::Output {
        self.add(Self::new(rhs.0, rhs.1))
    }
}

impl<T: ops::AddAssign + Copy> ops::AddAssign for Point<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: ops::AddAssign + Copy> ops::AddAssign<RawPoint<T>> for Point<T> {
    fn add_assign(&mut self, rhs: RawPoint<T>) {
        self.add_assign(Self::new(rhs.0, rhs.1))
    }
}

impl<T: ops::Sub<Output = T> + Copy> ops::Sub for Point<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T: ops::Sub<Output = T> + Copy> ops::Sub<RawPoint<T>> for Point<T> {
    type Output = Self;

    fn sub(self, rhs: RawPoint<T>) -> Self::Output {
        self.sub(Self::new(rhs.0, rhs.1))
    }
}

impl<T: ops::SubAssign + Copy> ops::SubAssign for Point<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: ops::SubAssign + Copy> ops::SubAssign<RawPoint<T>> for Point<T> {
    fn sub_assign(&mut self, rhs: RawPoint<T>) {
        self.sub_assign(Self::new(rhs.0, rhs.1))
    }
}

impl<T: ops::Mul<Output = T> + Copy> ops::Mul for Point<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl<T: ops::Mul<Output = T> + Copy> ops::Mul<RawPoint<T>> for Point<T> {
    type Output = Self;

    fn mul(self, rhs: RawPoint<T>) -> Self::Output {
        self.mul(Self::new(rhs.0, rhs.1))
    }
}

impl<T: ops::MulAssign + Copy> ops::MulAssign for Point<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl<T: ops::MulAssign + Copy> ops::MulAssign<RawPoint<T>> for Point<T> {
    fn mul_assign(&mut self, rhs: RawPoint<T>) {
        self.mul_assign(Point::new(rhs.0, rhs.1))
    }
}

impl<T: ops::Div<Output = T> + Copy> ops::Div for Point<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Point::new(self.x / rhs.x, self.y / rhs.y)
    }
}

impl<T: ops::Div<Output = T> + Copy> ops::Div<RawPoint<T>> for Point<T> {
    type Output = Self;

    fn div(self, rhs: RawPoint<T>) -> Self::Output {
        self.div(Point::new(rhs.0, rhs.1))
    }
}

impl<T: ops::DivAssign + Copy> ops::DivAssign for Point<T> {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl<T: ops::DivAssign + Copy> ops::DivAssign<RawPoint<T>> for Point<T> {
    fn div_assign(&mut self, rhs: RawPoint<T>) {
        self.div_assign(Point::new(rhs.0, rhs.1))
    }
}

pub type Dimen<T = u32> = Point<T>;

impl<T: Copy> Dimen<T> {
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
        let p1 = Point::new(0, 0);
        let p2 = Point::new(1, 2);
        let p3 = Point::new(3, 4);
        let p4 = Point::new(4, 6);
        let r2 = (1, 2);
        let r3 = (3, 4);
        assert_eq!(p2, p1 + p2);
        assert_eq!(p4, p2 + p3);
        assert_eq!(p2, p1 + r2);
        assert_eq!(p4, p2 + r3);
    }
}
