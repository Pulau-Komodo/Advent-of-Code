pub struct CartesianProduct<IA, IB, TB> {
	original_a: IA,
	a: IA,
	b: IB,
	current_b: Option<TB>,
}

impl<IA, IB, TB> CartesianProduct<IA, IB, TB>
where
	IA: Clone,
	TB: Clone,
{
	fn new(a: IA, b: IB) -> Self {
		Self {
			original_a: a.clone(),
			a,
			b,
			current_b: None,
		}
	}
}

impl<IA, IB, TA, TB> Iterator for CartesianProduct<IA, IB, TB>
where
	IA: Iterator<Item = TA> + Clone,
	IB: Iterator<Item = TB>,
	TB: Clone,
{
	type Item = (TA, TB);
	fn next(&mut self) -> Option<Self::Item> {
		let a = if let Some(a) = self.a.next() {
			a
		} else {
			self.a = self.original_a.clone();
			self.current_b = self.b.next();
			self.a.next()?
		};
		let b = if let Some(b) = self.current_b.clone() {
			b
		} else {
			let b = self.b.next()?;
			self.current_b = Some(b.clone());
			b
		};
		Some((a, b))
	}
}

pub trait IntoCartesianProduct<IA, IB, TB, O>
where
	IA: Iterator + Clone,
	IB: Iterator<Item = TB>,
	TB: Clone,
	O: IntoIterator<IntoIter = IB>,
{
	fn cartesian_product(self, other: O) -> CartesianProduct<IA, IB, TB>;
}

impl<IA, IB, TB, O> IntoCartesianProduct<IA, IB, TB, O> for IA
where
	IA: Iterator + Clone,
	IB: Iterator<Item = TB>,
	TB: Clone,
	O: IntoIterator<IntoIter = IB>,
{
	fn cartesian_product(self, other: O) -> CartesianProduct<IA, IB, TB> {
		CartesianProduct::new(self, other.into_iter())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_into() {
		let product: Vec<_> = ['a', 'b', 'c', 'd']
			.into_iter()
			.cartesian_product([1, 2, 3, 4])
			.collect();
		assert_eq!(
			product,
			vec![
				('a', 1),
				('b', 1),
				('c', 1),
				('d', 1),
				('a', 2),
				('b', 2),
				('c', 2),
				('d', 2),
				('a', 3),
				('b', 3),
				('c', 3),
				('d', 3),
				('a', 4),
				('b', 4),
				('c', 4),
				('d', 4)
			]
		);
	}

	#[test]
	fn test_range() {
		let product: Vec<_> = ['a', 'b', 'c', 'd']
			.into_iter()
			.cartesian_product(1..=4)
			.collect();
		assert_eq!(
			product,
			vec![
				('a', 1),
				('b', 1),
				('c', 1),
				('d', 1),
				('a', 2),
				('b', 2),
				('c', 2),
				('d', 2),
				('a', 3),
				('b', 3),
				('c', 3),
				('d', 3),
				('a', 4),
				('b', 4),
				('c', 4),
				('d', 4)
			]
		);
	}
}
