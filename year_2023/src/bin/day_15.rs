use std::array;

fn main() {
	shared::print_answers(15, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	input.trim().split(',').map(hash).map(|n| n as u32).sum()
}

fn get_answer_2(input: &str) -> u32 {
	let mut boxes: [Vec<Lens>; 256] = array::from_fn(|_| Vec::new());
	for step in input.trim().split(',').map(Step::from_str) {
		step.apply(&mut boxes);
	}
	boxes
		.into_iter()
		.enumerate()
		.map(|(box_num, lens_box)| {
			(box_num + 1)
				* lens_box
					.into_iter()
					.enumerate()
					.map(|(lens_pos, lens)| (lens_pos + 1) * lens.focal_length as usize)
					.sum::<usize>()
		})
		.sum::<usize>() as u32
}

fn hash(str: &str) -> u8 {
	let mut hash = 0_u16;
	for byte in str.bytes() {
		hash += byte as u16;
		hash *= 17;
		hash %= 256;
	}
	hash as u8
}

struct Step<'s> {
	label: &'s str,
	operation: Operation,
}

impl<'s> Step<'s> {
	fn from_str(str: &'s str) -> Self {
		if let Some((label, number)) = str.split_once('=') {
			let operation = Operation::Insert(number.parse().unwrap());
			Self { label, operation }
		} else {
			let label = &str[0..str.len() - 1];
			let operation = Operation::Remove;
			Self { label, operation }
		}
	}
	fn hash(&self) -> u8 {
		hash(self.label)
	}
	fn apply(&self, lens_boxes: &mut [Vec<Lens<'s>>]) {
		let lens_box = &mut lens_boxes[self.hash() as usize];
		match self.operation {
			Operation::Insert(focal_length) => {
				let mut found_lens = false;
				for lens in &mut *lens_box {
					if lens.label == self.label {
						lens.focal_length = focal_length;
						found_lens = true;
						break;
					}
				}
				if !found_lens {
					lens_box.push(Lens {
						label: self.label,
						focal_length,
					});
				}
			}
			Operation::Remove => {
				if let Some(lens_pos) = lens_box.iter().position(|lens| lens.label == self.label) {
					lens_box.remove(lens_pos);
				}
			}
		}
	}
}

enum Operation {
	Remove,
	Insert(u8),
}

struct Lens<'s> {
	label: &'s str,
	focal_length: u8,
}
