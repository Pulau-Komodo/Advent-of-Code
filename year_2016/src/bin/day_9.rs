fn main() {
	shared::print_answers(9, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> usize {
	let input = input.as_bytes();
	let mut length = 0;
	let mut position = 0;
	let mut marker: Option<String> = None;
	while position < input.len() {
		if let Some(marker_string) = &mut marker {
			if input[position] == b')' {
				let marker_struct = Marker::try_from_str(marker_string).unwrap();
				length += marker_struct.decompressed_length();
				position += marker_struct.length;
				marker = None;
			} else {
				marker_string.push(char::from(input[position]));
			}
		} else if input[position] == b'(' {
			marker = Some(String::new());
		} else {
			length += 1;
		}
		position += 1;
	}
	length
}

fn get_answer_2(input: &str) -> usize {
	let input = input.as_bytes();
	let mut length = 0;
	let mut position = 0;
	let mut marker: Option<String> = None;
	let mut markers: Vec<(usize, Marker)> = Vec::new();
	while position < input.len() {
		if let Some(marker_string) = &mut marker {
			if input[position] == b')' {
				let marker_struct = Marker::try_from_str(marker_string).unwrap();
				markers.push((position, marker_struct));
				marker = None;
			} else {
				marker_string.push(char::from(input[position]));
			}
		} else if input[position] == b'(' {
			marker = Some(String::new());
		} else {
			let mut repetition = 1;
			for (_, marker) in &markers {
				repetition *= marker.repetition;
			}
			length += repetition;
		}
		position += 1;
		drain_filter(&mut markers, |(starting_pos, marker)| {
			position > *starting_pos + marker.length
		});
	}
	length
}

struct Marker {
	length: usize,
	repetition: usize,
}

impl Marker {
	fn try_from_str(str: &str) -> Option<Self> {
		let (length, repetition) = str.split_once('x')?;
		Some(Self {
			length: length.parse().ok()?,
			repetition: repetition.parse().ok()?,
		})
	}
	fn decompressed_length(&self) -> usize {
		self.length * self.repetition
	}
}

fn drain_filter<T, F>(vec: &mut Vec<T>, predicate: F)
where
	F: Fn(&T) -> bool,
{
	let mut i = 0;
	while i < vec.len() {
		if predicate(&vec[i]) {
			vec.remove(i);
		} else {
			i += 1;
		}
	}
}
