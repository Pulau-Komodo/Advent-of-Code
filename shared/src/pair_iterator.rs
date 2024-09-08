pub struct PairIterator<I, T>
where
	I: Iterator<Item = T>,
{
	item: Option<T>,
	iterator: I,
}

impl<I, T> Iterator for PairIterator<I, T>
where
	I: Iterator<Item = T>,
	T: Copy,
{
	type Item = (T, T);
	fn next(&mut self) -> Option<Self::Item> {
		match (self.item, self.iterator.next()) {
			(Some(item), Some(next)) => {
				self.item = Some(next);
				Some((item, next))
			}
			_ => None,
		}
	}
}

pub trait IntoPairIterator<I, T>
where
	I: Iterator<Item = T>,
	T: Copy,
{
	fn pairs(self) -> PairIterator<I, T>;
}

impl<I, T> IntoPairIterator<I, T> for I
where
	I: Iterator<Item = T>,
	T: Copy,
{
	fn pairs(mut self) -> PairIterator<I, T> {
		PairIterator {
			item: self.next(),
			iterator: self,
		}
	}
}
