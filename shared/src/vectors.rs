use std::{
	any::type_name,
	fmt::Debug,
	iter::Product,
	ops::{Add, Div, Mul, Rem, Sub},
	str::FromStr,
};

use crate::internal::one;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Vec3<T> {
	pub x: T,
	pub y: T,
	pub z: T,
}

impl<T> Vec3<T> {
	pub const fn new(x: T, y: T, z: T) -> Self {
		Self { x, y, z }
	}
}

impl<T> Vec3<T>
where
	T: Add<Output = T> + Sub<Output = T> + Product + Copy,
{
	pub fn orthogonal_neighbours(self) -> [Self; 6] {
		let Self { x, y, z } = self;
		let one = one();
		[
			Self { x: x - one, y, z },
			Self { x: x + one, y, z },
			Self { x, y: y - one, z },
			Self { x, y: y + one, z },
			Self { x, y, z: z - one },
			Self { x, y, z: z + one },
		]
	}
}

impl<T> Vec3<T>
where
	T: FromStr,
	<T as FromStr>::Err: Debug,
{
	pub fn from_comma_separated(str: &str) -> Self {
		let mut parts = str.split(',').map(|str| str.parse().unwrap());
		Self {
			x: parts.next().unwrap(),
			y: parts.next().unwrap(),
			z: parts.next().unwrap(),
		}
	}
}

impl<T> Add for Vec3<T>
where
	T: Add<Output = T>,
{
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
			z: self.z + rhs.z,
		}
	}
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Vec2<T> {
	pub x: T,
	pub y: T,
}

impl<T> Vec2<T> {
	pub const fn new(x: T, y: T) -> Self {
		Self { x, y }
	}
}

impl<T> Vec2<T>
where
	T: Copy,
{
	pub const fn swap_xy(self) -> Self {
		Self {
			x: self.y,
			y: self.x,
		}
	}
}

impl<T> Vec2<T>
where
	T: Add<Output = T> + Sub<Output = T> + Product + Copy,
{
	pub fn orthogonal_neighbours(self) -> [Self; 4] {
		let Self { x, y } = self;
		let one = one();
		[
			Self { x: x - one, y },
			Self { x: x + one, y },
			Self { x, y: y - one },
			Self { x, y: y + one },
		]
	}
}

impl<T> Vec2<T>
where
	T: FromStr,
	<T as FromStr>::Err: Debug,
{
	pub fn from_comma_separated(str: &str) -> Self {
		let mut parts = str.split(',').map(|str| str.parse().unwrap());
		Self {
			x: parts.next().unwrap(),
			y: parts.next().unwrap(),
		}
	}
}

impl<T> Add for Vec2<T>
where
	T: Add<Output = T>,
{
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
		}
	}
}

impl<T> Sub for Vec2<T>
where
	T: Sub<Output = T>,
{
	type Output = Self;
	fn sub(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
		}
	}
}

impl<T> Div for Vec2<T>
where
	T: Div<Output = T>,
{
	type Output = Self;
	fn div(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x / rhs.x,
			y: self.y / rhs.y,
		}
	}
}

impl<T> Rem for Vec2<T>
where
	T: Rem<Output = T>,
{
	type Output = Self;
	fn rem(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x % rhs.x,
			y: self.y % rhs.y,
		}
	}
}

impl<T> Mul for Vec2<T>
where
	T: Mul<Output = T>,
{
	type Output = Self;
	fn mul(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x * rhs.x,
			y: self.y * rhs.y,
		}
	}
}

impl<T> Div<T> for Vec2<T>
where
	T: Div<Output = T> + Copy,
{
	type Output = Self;
	fn div(self, rhs: T) -> Self::Output {
		Self {
			x: self.x / rhs,
			y: self.y / rhs,
		}
	}
}

impl<T> Rem<T> for Vec2<T>
where
	T: Rem<Output = T> + Copy,
{
	type Output = Self;
	fn rem(self, rhs: T) -> Self::Output {
		Self {
			x: self.x % rhs,
			y: self.y % rhs,
		}
	}
}

impl<T> Mul<T> for Vec2<T>
where
	T: Mul<Output = T> + Copy,
{
	type Output = Self;
	fn mul(self, rhs: T) -> Self::Output {
		Self {
			x: self.x * rhs,
			y: self.y * rhs,
		}
	}
}

impl<T> Debug for Vec2<T>
where
	T: Debug,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!(
			"Vec2<{}> {{ x: {:?}, y: {:?} }}",
			type_name::<T>(),
			self.x,
			self.y
		))
	}
}

impl Vec2<usize> {
	pub const ONE: Vec2<usize> = Vec2 { x: 1, y: 1 };
}
