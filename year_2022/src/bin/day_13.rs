use std::cmp::Ordering;

use shared::bytes_to_integer;

fn main() {
	shared::print_answers(13, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> usize {
	input
		.split("\n\n")
		.map(|pair| {
			let (left, right) = pair.split_once('\n').unwrap();
			(Value::from_str(left), Value::from_str(right))
		})
		.enumerate()
		.filter_map(|(index, (left, right))| (left < right).then_some(index + 1))
		.sum()
}

fn get_answer_2(input: &str) -> usize {
	let mut divider_indices = [1, 2];
	let divider_packets = ["[[2]]", "[[6]]"].map(Value::from_str);
	for packet in input
		.split('\n')
		.filter(|line| !line.is_empty())
		.map(Value::from_str)
	{
		if packet < divider_packets[1] {
			divider_indices[1] += 1;
			if packet < divider_packets[0] {
				divider_indices[0] += 1;
			}
		}
	}
	divider_indices.into_iter().product()
}

#[derive(Clone, PartialEq, Eq)]
enum Value {
	List(Vec<Value>),
	Integer(u8),
}

impl Value {
	fn from_str(str: &str) -> Self {
		Self::from_bytes(str.as_bytes())
	}
	fn from_bytes(bytes: &[u8]) -> Self {
		if bytes.is_empty() {
			return Self::List(Vec::new());
		}
		if bytes[0] != b'[' {
			return Self::Integer(bytes_to_integer(bytes));
		}
		let mut open_brackets: u8 = 1;
		let list: Vec<_> = bytes[1..bytes.len() - 1]
			.split(|&byte| {
				if byte == b',' && open_brackets == 1 {
					return true;
				}
				match byte {
					b'[' => open_brackets += 1,
					b']' => open_brackets -= 1,
					_ => (),
				};
				false
			})
			.map(Self::from_bytes)
			.collect();
		if list.len() == 1 && matches!(list.first(), Some(Self::Integer(_))) {
			list.into_iter().next().unwrap()
		} else {
			Self::List(list)
		}
	}
}

impl PartialOrd for Value {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Value {
	/// This can be done much more nicely by using lexographic ordering. But it didn't feel right to use it when I only learned about it after solving it, and it seems incidental that the sorting behaviour even matches.
	fn cmp(&self, other: &Self) -> Ordering {
		use Value::*;
		match (self, other) {
			(Integer(left), Integer(right)) => left.cmp(right),
			(List(left), List(right)) => {
				if let Some(outcome) = left.iter().zip(right).find_map(|(left, right)| {
					let ordering = left.cmp(right);
					(ordering != Ordering::Equal).then_some(ordering)
				}) {
					outcome
				} else {
					left.len().cmp(&right.len())
				}
			}
			(Integer(_), List(right)) => {
				let outcome = Some(self).cmp(&right.first());
				if outcome != Ordering::Equal {
					outcome
				} else {
					1.cmp(&right.len())
				}
			}
			(List(left), Integer(_)) => {
				let outcome = left.first().cmp(&Some(other));
				if outcome != Ordering::Equal {
					outcome
				} else {
					left.len().cmp(&1)
				}
			}
		}
	}
}

impl core::fmt::Debug for Value {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::List(arg0) => f.write_fmt(format_args!("{:?}", arg0)),
			Self::Integer(arg0) => f.write_fmt(format_args!("{arg0}")),
		}
	}
}
