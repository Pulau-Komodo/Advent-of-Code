use std::{
	iter::Product,
	ops::{Add, Div, DivAssign, Range, Rem, Sub},
};

pub fn div_ceil<T>(num: T, rhs: T) -> T
where
	T: Copy
		+ Add<Output = T>
		+ Div<Output = T>
		+ Rem<Output = T>
		+ Default
		+ From<bool>
		+ PartialEq,
{
	num / rhs + (num % rhs != T::default()).into()
}

pub fn wrapping_add<T>(num: T, rhs: T, range: Range<T>) -> T
where
	T: Copy + Add<Output = T> + Sub<Output = T> + Rem<Output = T>,
{
	(num + rhs - range.start) % (range.end - range.start) + range.start
}

pub fn wrapping_sub<T>(num: T, rhs: T, range: Range<T>) -> T
where
	T: Copy + Add<Output = T> + Sub<Output = T> + Rem<Output = T>,
{
	(num + range.end - range.start - range.start - rhs) % (range.end - range.start) + range.start
}

/// This will only make sense on integer types.
pub fn count_digits<T: DivAssign + Clone + Default + Product + PartialEq>(mut n: T, base: T) -> u8 {
	let zero = T::default();
	let mut digits = 0;
	while n != zero {
		n /= base.clone();
		digits += 1;
	}
	digits.max(1)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn wrapping() {
		assert_eq!(wrapping_add(5, 30, 0..10), 5);
		assert_eq!(wrapping_add(6, 1, 5..7), 5);
		assert_eq!(wrapping_add(6, 2, 4..7), 5);
		assert_eq!(wrapping_add(150, 0, 100..200), 150);
		assert_eq!(wrapping_sub::<u8>(5, 10, 0..10), 5);
		assert_eq!(wrapping_sub(6, 1, 5..7), 5);
		assert_eq!(wrapping_sub(6, 1, 5..10), 5);
		assert_eq!(wrapping_sub(6, 2, 5..10), 9);
		assert_eq!(wrapping_sub(6, 3, 5..10), 8);
		assert_eq!(wrapping_sub(150, 0, 100..200), 150);
	}
	#[test]
	fn digit_counting() {
		let digits = [0, 1, 9, 10, 12345, u128::MAX].map(|n| count_digits(n, 10));
		assert_eq!(digits, [1, 1, 1, 2, 5, 39]);
	}
	#[test]
	fn digit_counting_binary() {
		let digits =
			[0b0, 0b1, 0b111, 0b1000, 0b11000000111001, u128::MAX].map(|n| count_digits(n, 2));
		assert_eq!(digits, [1, 1, 3, 4, 14, 128]);
	}
}
