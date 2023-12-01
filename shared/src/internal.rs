use std::iter::Product;

#[inline]
pub fn one<T: Product>() -> T {
	None::<T>.into_iter().product()
}
