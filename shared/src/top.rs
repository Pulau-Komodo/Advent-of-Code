pub trait IteratorTop<T>
where
	Self: Iterator<Item = T>,
	T: Ord,
{
	fn top<const L: usize>(&mut self) -> [T; L];
	fn top_binary<const L: usize>(&mut self) -> [T; L];
}

impl<I, T> IteratorTop<T> for I
where
	I: Iterator<Item = T>,
	T: Ord,
{
	/// Get the top L items from the iterator, where L is specified by the const generic. They will be sorted low to high.
	///
	/// This one internally uses a linear search to know where to insert a new item. For large values of L, you might want `top_binary`.
	///
	/// # Panics
	/// Panics if there aren't at least L items in the iterator.
	fn top<const L: usize>(&mut self) -> [T; L] {
		let mut top = [(); L].map(|_| self.next().unwrap());
		top.sort_unstable();
		for element in self {
			if let Some(insertion_point) = (0..L).take_while(|i| element > top[*i]).last() {
				for i in 0..insertion_point {
					top.swap(i, i + 1);
				}
				top[insertion_point] = element;
			}
		}
		top
	}
	/// Get the top L items from the iterator, where L is specified by the const generic. They will be sorted low to high.
	///
	/// This one internally uses a binary search to know where to insert a new item. For small values of L, you might want `top`.
	///
	/// # Panics
	/// Panics if there aren't at least L items in the iterator.
	fn top_binary<const L: usize>(&mut self) -> [T; L] {
		let mut top = [(); L].map(|_| self.next().unwrap());
		top.sort_unstable();
		for element in self {
			let insertion_point = top.partition_point(|n| n < &element);
			if insertion_point > 0 {
				let insertion_point = insertion_point - 1;
				for i in 0..insertion_point {
					top.swap(i, i + 1);
				}
				top[insertion_point] = element;
			}
		}
		top
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	#[should_panic]
	fn panic_if_too_short() {
		[4, 1, 2, 3].iter().top::<5>();
	}
	#[test]
	fn no_panic_if_equal_size() {
		assert_eq!([&1, &2, &3, &4], [4, 1, 2, 3].iter().top::<4>());
	}
	#[test]
	fn basic_functioning() {
		let list = [
			21, 1, 62, 27, 77, 44, 37, 79, 38, 36, 77, 20, 60, 72, 73, 4, 58, 54, 70, 69,
		];
		assert_eq!([&73, &77, &77, &79], list.iter().top::<4>());
	}
	#[test]
	fn basic_functioning_binary() {
		let list = [
			81, 22, 69, 70, 94, 23, 38, 12, 88, 81, 44, 60, 12, 40, 52, 12, 23, 12, 47, 18,
		];
		assert_eq!(
			[&52, &60, &69, &70, &81, &81, &88, &94],
			list.iter().top_binary::<8>()
		);
	}
}
