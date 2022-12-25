use std::{
	iter::Product,
	ops::{AddAssign, MulAssign},
};

/// Parses the bytes of a slice as one number, assuming all the bytes represent numerical ASCII characters, in reading order. If that is not the case, the result is probably nonsense.
///
/// Due to assuming all bytes are numerical, it also cannot handle negative inputs.
///
/// Due to a lack of numeric trait bounds, this could technically be used with non-numerical `T`s, but the outcome is very unlikely to make sense.
pub fn bytes_to_integer<T>(bytes: &[u8]) -> T
where
	T: From<u8> + Copy + AddAssign + MulAssign + Product + Default,
{
	let mut number = T::default();
	for byte in bytes {
		number *= 10.into();
		number += (byte - b'0').into();
	}
	number
}

/// Starting from the left, parses a string of ASCII digits into a number until it finds a non-numerical byte. Then returns the number and the remainder of the string slice.
///
/// # Panics
/// Panics if not a single digit was found.
pub fn split_number<T>(string: &str) -> (T, &str)
where
	T: Default + From<u8> + AddAssign + MulAssign,
{
	let mut number = T::default();
	for (index, byte) in string.bytes().enumerate() {
		match byte {
			b'0'..=b'9' => {
				number *= 10.into();
				number += (byte - b'0').into();
			}
			_ => {
				if index != 0 {
					return (number, &string[index..]);
				} else {
					panic!("No numerical character found.")
				}
			}
		}
	}
	(number, "")
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
	#[test]
	fn split_num() {
		assert_eq!(split_number("15 test"), (15, " test"));
	}
}
