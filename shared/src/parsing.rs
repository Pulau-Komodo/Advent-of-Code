use std::{
	iter::Product,
	ops::{Add, Mul},
};

use crate::internal::ten;

/// Parses the bytes of a slice as one number, assuming all the bytes represent numerical ASCII characters, in reading order. If that is not the case, the result is probably nonsense.
///
/// Due to assuming all bytes are numerical, it also cannot handle negative inputs.
///
/// Due to a lack of numeric trait bounds, this could technically be used with non-numerical `T`s, but the outcome is very unlikely to make sense.
pub fn bytes_to_integer<T>(bytes: &[u8]) -> T
where
	T: From<u8> + Copy + Add<Output = T> + Mul<Output = T> + Product + Default,
{
	let mut number = T::default();
	for byte in bytes {
		number = number * ten();
		number = number + (byte - b'0').into();
	}
	number
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse_bytes() {
		assert_eq!(25_u8, bytes_to_integer("25".as_bytes()));
		assert_eq!(500_u16, bytes_to_integer("500".as_bytes()));
		assert_eq!(98765_i32, bytes_to_integer("98765".as_bytes()));
	}
}
