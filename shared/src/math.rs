use std::ops::{Add, Div, Range, Rem, Sub};

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
}
