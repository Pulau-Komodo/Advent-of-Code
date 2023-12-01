use std::ops::{
	BitAnd, BitAndAssign, BitOrAssign, BitXor, BitXorAssign, Deref, DerefMut, Not, Shl,
};

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Bitmask<T> {
	pub bitmask: T,
}

impl<T> Bitmask<T>
where
	T: Default,
{
	pub fn new() -> Self {
		Self {
			bitmask: T::default(),
		}
	}
}

impl<T> Bitmask<T>
where
	T: Copy + From<bool> + Shl<usize, Output = T> + BitAnd<Output = T> + PartialEq,
{
	pub fn get(&self, index: usize) -> bool {
		T::from(true) << index & self.bitmask != T::from(false)
	}
}

impl<T> Bitmask<T>
where
	T: From<bool> + BitOrAssign + BitAndAssign + Not<Output = T> + Shl<usize, Output = T>,
{
	pub fn set(&mut self, index: usize, state: bool) {
		if state {
			self.bitmask |= T::from(true) << index
		} else {
			self.bitmask &= !(T::from(true) << index)
		}
	}
}

impl<T> Bitmask<T>
where
	T: Copy
		+ From<bool>
		+ Shl<usize, Output = T>
		+ BitAnd<Output = T>
		+ PartialEq
		+ BitAndAssign
		+ BitOrAssign
		+ Not<Output = T>,
{
	pub fn get_mut(&mut self, index: usize) -> Bit<T> {
		let current = self.get(index);
		Bit {
			bitmask: self,
			bool: current,
			bit_bitmask: T::from(true) << index,
		}
	}
}

pub struct Bit<'l, T>
where
	T: Copy + BitAndAssign + BitOrAssign + Not<Output = T>,
{
	bitmask: &'l mut Bitmask<T>,
	bool: bool,
	bit_bitmask: T,
}

impl<'l, T> Deref for Bit<'l, T>
where
	T: Copy + BitAndAssign + BitOrAssign + Not<Output = T>,
{
	type Target = bool;
	fn deref(&self) -> &Self::Target {
		&self.bool
	}
}

impl<'l, T> DerefMut for Bit<'l, T>
where
	T: Copy + BitAndAssign + BitOrAssign + Not<Output = T>,
{
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.bool
	}
}

impl<'l, T> Drop for Bit<'l, T>
where
	T: Copy + BitAndAssign + BitOrAssign + Not<Output = T>,
{
	fn drop(&mut self) {
		if self.bool {
			self.bitmask.bitmask |= self.bit_bitmask;
		} else {
			self.bitmask.bitmask &= !self.bit_bitmask;
		}
	}
}

impl<'l, T> BitXor<bool> for Bit<'l, T>
where
	T: Copy + BitAndAssign + BitOrAssign + Not<Output = T>,
{
	type Output = Self;
	fn bitxor(mut self, rhs: bool) -> Self::Output {
		self.bool ^= rhs;
		self
	}
}

impl<'l, T> BitXorAssign<bool> for Bit<'l, T>
where
	T: Copy + BitAndAssign + BitOrAssign + Not<Output = T>,
{
	fn bitxor_assign(&mut self, rhs: bool) {
		self.bool ^= rhs;
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn bit_xor() {
		let mut bitmask = Bitmask::<u16>::new();
		for index in (0..16).step_by(2) {
			let mut bit = bitmask.get_mut(index);
			bit ^= true;
		}
		assert_eq!(bitmask.bitmask, 0b01010101_01010101);
		for index in (0..16).step_by(4) {
			let mut bit = bitmask.get_mut(index);
			bit ^= true;
		}
		assert_eq!(bitmask.bitmask, 0b01000100_01000100);
	}
	#[test]
	fn deref_mut() {
		let mut bitmask = Bitmask::<u16>::new();
		for index in (0..16).step_by(2) {
			let mut bit = bitmask.get_mut(index);
			*bit |= true;
		}
		assert_eq!(bitmask.bitmask, 0b01010101_01010101);
		for index in (0..16).step_by(4) {
			let mut bit = bitmask.get_mut(index);
			*bit &= false;
		}
		assert_eq!(bitmask.bitmask, 0b01000100_01000100);
	}
}
