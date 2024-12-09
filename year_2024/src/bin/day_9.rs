use std::cmp::Ordering;

fn main() {
	shared::print_answers(9, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u64 {
	let mut disk_map = DiskMap::from_input(input);
	while disk_map.compact() {}
	disk_map.checksum()
}

fn get_answer_2(input: &str) -> u64 {
	let mut disk_map = DiskMap::from_input(input);
	disk_map.compact_v2();
	disk_map.checksum()
}

// Regretting my representation but I have things to do.
#[derive(Debug, Clone, Copy)]
struct Span {
	length: u8,
	id: Option<usize>,
}

impl Span {
	fn split(self, point: u8) -> (Self, Self) {
		let Self { length, id } = self;
		(
			Self { length: point, id },
			Self {
				length: length - point,
				id,
			},
		)
	}
}

struct DiskMap {
	spans: Vec<Span>,
}

impl DiskMap {
	fn from_input(input: &str) -> Self {
		let mut file = true;
		let mut id = 0;
		let spans = input
			.trim()
			.bytes()
			.filter_map(|byte| {
				let length = byte - b'0';
				let output = if file {
					let file = Span {
						length,
						id: Some(id),
					};
					id += 1;
					file
				} else {
					Span { length, id: None }
				};
				file = !file;
				(length > 0).then_some(output)
			})
			.collect();
		Self { spans }
	}
	fn compact(&mut self) -> bool {
		let mut file = {
			let span = self.spans.pop().unwrap();
			if span.id.is_some() {
				span
			} else {
				self.spans.pop().unwrap()
			}
		};
		loop {
			let Some((index, first_gap)) = self
				.spans
				.iter_mut()
				.enumerate()
				.find(|(_, span)| span.id.is_none())
			else {
				self.spans.push(file);
				return false;
			};
			match first_gap.length.cmp(&file.length) {
				Ordering::Less => {
					let (a, b) = file.split(first_gap.length);
					first_gap.id = a.id;
					file = b;
					continue;
				}
				Ordering::Equal => {
					first_gap.id = file.id;
				}
				Ordering::Greater => {
					first_gap.length -= file.length;
					self.spans.insert(index, file);
				}
			}
			break;
		}
		true
	}
	fn compact_v2(&mut self) {
		let highest_file_id = self.spans.iter().rev().find_map(|span| span.id).unwrap();
		for file_id in (0..=highest_file_id).rev() {
			let file_index = self
				.spans
				.iter()
				.position(|span| span.id == Some(file_id))
				.unwrap();
			let length = self.spans[file_index].length;
			if let Some(gap_index) = self
				.spans
				.iter()
				.position(|span| span.id.is_none() && span.length >= length)
			{
				if gap_index > file_index {
					continue;
				}
				let file = self.spans[file_index];
				self.spans[file_index].id = None;
				self.consolidate_gaps(file_index);
				let gap = &mut self.spans[gap_index];
				if gap.length == length {
					gap.id = Some(file_id);
				} else {
					gap.length -= length;
					self.spans.insert(gap_index, file);
				}
			}
		}
	}
	fn consolidate_gaps(&mut self, index: usize) {
		let middle = self.spans[index];
		let left = index
			.checked_sub(1)
			.and_then(|i| self.spans.get(i).filter(|span| span.id.is_none()));
		let right = self.spans.get(index + 1).filter(|span| span.id.is_none());
		match (left, right) {
			(Some(l), Some(r)) => {
				self.spans[index - 1].length = l.length + middle.length + r.length;
				self.spans.remove(index);
				self.spans.remove(index);
			}
			(Some(l), None) => {
				self.spans[index - 1].length = l.length + middle.length;
				self.spans.remove(index);
			}
			(None, Some(r)) => {
				self.spans[index].length = middle.length + r.length;
				self.spans.remove(index + 1);
			}
			(None, None) => (),
		}
	}
	fn checksum(&self) -> u64 {
		let mut block = 0;
		let mut sum = 0;
		for span in &self.spans {
			if let Some(id) = span.id {
				for _ in 0..span.length {
					sum += block * id as u64;
					block += 1;
				}
			} else {
				block += span.length as u64;
			}
		}
		sum
	}
}
