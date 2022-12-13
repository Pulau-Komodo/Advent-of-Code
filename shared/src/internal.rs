use std::{iter::Product, ops::Add};

#[inline]
pub fn one<T: Product>() -> T {
	None::<T>.into_iter().product()
}

#[inline]
pub fn ten<T: Copy + Product + Add<Output = T>>() -> T {
	let one: T = one();
	one + one + one + one + one + one + one + one + one + one
}
