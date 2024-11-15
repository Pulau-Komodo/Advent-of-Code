fn main() {
	shared::print_answers(8, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	let mut numbers = input
		.trim()
		.split(' ')
		.map(|num| num.parse::<u32>().unwrap());

	let root = Node::from_iterator(&mut numbers);
	let mut node_stack = vec![root];
	let mut metadata_sum = 0;
	while !node_stack.is_empty() {
		let node = node_stack.last_mut().unwrap();
		if node.child_count == 0 {
			metadata_sum += take_exact(&mut numbers, node.metadatum_count as usize).sum::<u32>();
			node_stack.pop().unwrap();
		} else {
			node.child_count -= 1;

			let new_node = Node::from_iterator(&mut numbers);
			node_stack.push(new_node);
		}
	}
	metadata_sum
}

fn get_answer_2(input: &str) -> u32 {
	let mut numbers = input
		.trim()
		.split(' ')
		.map(|num| num.parse::<u32>().unwrap());

	let root = NodeV2::from_iterator(&mut numbers);
	let mut node_stack = vec![root];
	loop {
		let node = node_stack.last_mut().unwrap();
		if node.child_count == 0 {
			let finished_node = node_stack.pop().unwrap();
			let value = finished_node.get_value(&mut numbers);
			let Some(new_top_node) = node_stack.last_mut() else {
				return value;
			};
			new_top_node.set_top_child_value(value);
		} else {
			node.child_count -= 1;

			let new_node = NodeV2::from_iterator(&mut numbers);
			node_stack.push(new_node);
		}
	}
}

#[derive(Debug)]
struct Node {
	child_count: u32,
	metadatum_count: u32,
}

impl Node {
	fn from_iterator(numbers: &mut impl Iterator<Item = u32>) -> Self {
		let child_count = numbers.next().unwrap();
		let metadatum_count = numbers.next().unwrap();
		Self {
			child_count,
			metadatum_count,
		}
	}
}

#[derive(Debug)]
struct NodeV2 {
	/// The number of children left to process.
	child_count: u32,
	/// The number of metadatum numbers.
	metadatum_count: u32,
	/// The values of each of the children, initialised at 0 and filled out as they are discovered. Doubles as a way to track how many children there are total.
	child_values: Vec<u32>,
}

impl NodeV2 {
	fn from_iterator(numbers: &mut impl Iterator<Item = u32>) -> Self {
		let child_count = numbers.next().unwrap();
		let metadatum_count = numbers.next().unwrap();
		Self {
			child_count,
			metadatum_count,
			child_values: vec![0; child_count as usize],
		}
	}
	/// Either sum up the metadata, or sum up the value of the children indicated by the metadata.
	fn get_value(&self, numbers: &mut impl Iterator<Item = u32>) -> u32 {
		if self.child_values.is_empty() {
			take_exact(numbers, self.metadatum_count as usize).sum()
		} else {
			take_exact(numbers, self.metadatum_count as usize)
				.map(|child| {
					self.child_values
						.get(child as usize - 1)
						.copied()
						.unwrap_or(0)
				})
				.sum()
		}
	}
	fn set_top_child_value(&mut self, value: u32) {
		let total_child_count = self.child_values.len();
		self.child_values[total_child_count - self.child_count as usize - 1] += value;
	}
}

fn take_exact<I>(iterator: &mut I, n: usize) -> impl Iterator<Item = I::Item> + '_
where
	I: Iterator,
{
	(0..n).map(|_| iterator.next().unwrap())
}
