use std::{ops, cmp::Ordering};

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

	pub fn mag(&self) -> f32 {
		((self.x * self.x + self.y * self.y) as f32).sqrt()
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

impl Ord for Vector {
	fn cmp(&self, rhs: &Self) -> Ordering {
		match self.x.cmp(&rhs.x) {
			Ordering::Equal => self.y.cmp(&rhs.y),
			x => x
		}
    }
}

impl PartialOrd for Vector {
	fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
		match self.x.partial_cmp(&rhs.x) {
			Some(Ordering::Equal) => self.y.partial_cmp(&rhs.y),
			x => x
		}
	}
}