fn main() {
	shared::print_answers(8, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	let mut numbers = input
		.trim()
		.split(' ')
		.map(|num| num.parse::<u32>().unwrap());
	let mut node_stack = Vec::<Node>::new();
	let mut metadata_sum = 0;
	loop {
		if let Some(node) = node_stack.last_mut() {
			if node.child_count == 0 {
				metadata_sum +=
					take_exact(&mut numbers, node.metadatum_count as usize).sum::<u32>();
				node_stack.pop().unwrap();
				continue;
			}
			node.child_count -= 1;
		}
		let Some(new_node) = Node::try_from_iterator(&mut numbers) else {
			break;
		};
		node_stack.push(new_node);
	}
	metadata_sum
}

fn get_answer_2(input: &str) -> u32 {
	let mut numbers = input
		.trim()
		.split(' ')
		.map(|num| num.parse::<u32>().unwrap());
	let mut node_stack = Vec::<NodeV2>::new();
	loop {
		if let Some(node) = node_stack.last_mut() {
			if node.child_count == 0 {
				let finished_node = node_stack.pop().unwrap();
				let value = finished_node.get_value(&mut numbers);
				let Some(new_top_node) = node_stack.last_mut() else {
					return value;
				};
				let total_child_count = new_top_node.child_values.len();
				new_top_node.child_values
					[total_child_count - new_top_node.child_count as usize - 1] += value;
				continue;
			}
			node.child_count -= 1;
		}
		let new_node = NodeV2::try_from_iterator(&mut numbers).unwrap();
		node_stack.push(new_node);
	}
}

#[derive(Debug)]
struct Node {
	child_count: u32,
	metadatum_count: u32,
}

impl Node {
	fn try_from_iterator(numbers: &mut impl Iterator<Item = u32>) -> Option<Self> {
		let child_count = numbers.next()?;
		let metadatum_count = numbers.next().unwrap();
		Some(Self {
			child_count,
			metadatum_count,
		})
	}
}

#[derive(Debug)]
struct NodeV2 {
	child_count: u32,
	metadatum_count: u32,
	child_values: Vec<u32>,
}

impl NodeV2 {
	fn try_from_iterator(numbers: &mut impl Iterator<Item = u32>) -> Option<Self> {
		let child_count = numbers.next()?;
		let metadatum_count = numbers.next().unwrap();
		Some(Self {
			child_count,
			metadatum_count,
			child_values: vec![0; child_count as usize],
		})
	}
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
}

fn take_exact<I>(iterator: &mut I, n: usize) -> impl Iterator<Item = I::Item> + '_
where
	I: Iterator,
{
	(0..n).map(|_| iterator.next().unwrap())
}
