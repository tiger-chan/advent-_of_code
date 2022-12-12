use std::{cmp::Ordering, ops};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Vector {
    pub x: i32,
    pub y: i32,
}

impl Vector {
    pub fn new(x: i32, y: i32) -> Self {
        Vector { x: x, y: y }
    }

    pub fn up() -> Self {
        Vector::new(0, 1)
    }

    pub fn down() -> Self {
        Vector::new(0, -1)
    }

    pub fn right() -> Self {
        Vector::new(1, 0)
    }

    pub fn left() -> Self {
        Vector::new(-1, 0)
    }

    pub fn mag_sqr(&self) -> f32 {
        (self.x * self.x + self.y * self.y) as f32
    }

    #[allow(dead_code)]
    pub fn mag(&self) -> f32 {
        self.mag_sqr().sqrt()
    }

    pub fn clamp(v: Self, min: i32, max: i32) -> Self {
        Vector::new(
            i32::min(i32::max(min, v.x), max),
            i32::min(i32::max(min, v.y), max),
        )
    }

	pub fn abs(&self) -> Self {
		Vector::new(self.x.abs(), self.y.abs())
	}
}

impl ops::Add for Vector {
    type Output = Vector;
    fn add(self, rhs: Self) -> Self::Output {
        Vector::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl ops::Sub for Vector {
    type Output = Vector;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl ops::Neg for Vector {
    type Output = Vector;
    fn neg(self) -> Self::Output {
        Vector::new(-self.x, -self.y)
    }
}

impl ops::AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::SubAssign for Vector {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl ops::Mul for Vector {
    type Output = Vector;
    fn mul(self, rhs: Self) -> Self::Output {
        Vector::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl Ord for Vector {
    fn cmp(&self, rhs: &Self) -> Ordering {
        match self.x.cmp(&rhs.x) {
            Ordering::Equal => self.y.cmp(&rhs.y),
            x => x,
        }
    }
}

impl PartialOrd for Vector {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        match self.x.partial_cmp(&rhs.x) {
            Some(Ordering::Equal) => self.y.partial_cmp(&rhs.y),
            x => x,
        }
    }
}
