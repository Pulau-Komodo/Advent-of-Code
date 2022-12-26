fn main() {
	shared::print_labelled_answers(
		20,
		[
			("1 ordered      ", get_answer_1_ordered as fn(&str) -> i64),
			("2 ordered      ", get_answer_2_ordered),
			("1 doubly linked", get_answer_1_doubly_linked),
			("2 doubly linked", get_answer_2_doubly_linked),
			("1 singly linked", get_answer_1_singly_linked),
			("2 singly linked", get_answer_2_singly_linked),
			("1 indexed      ", get_answer_1_indexed),
			("2 indexed      ", get_answer_2_indexed),
		],
	);
}

fn get_answer_1_singly_linked(input: &str) -> i64 {
	let mut file = SinglyLinkedFile::from_str(input);
	file.mix();
	file.grove_coordinates()
}

fn get_answer_2_singly_linked(input: &str) -> i64 {
	let mut file = SinglyLinkedFile::from_str(input);
	file.multiply_numbers(DECRYPTION_KEY);
	for _ in 0..10 {
		file.mix();
	}
	file.grove_coordinates()
}

fn get_answer_1_doubly_linked(input: &str) -> i64 {
	let mut file = DoublyLinkedFile::from_str(input);
	file.mix();
	file.grove_coordinates()
}

fn get_answer_2_doubly_linked(input: &str) -> i64 {
	let mut file = DoublyLinkedFile::from_str(input);
	file.multiply_numbers(DECRYPTION_KEY);
	for _ in 0..10 {
		file.mix();
	}
	file.grove_coordinates()
}

fn get_answer_1_indexed(input: &str) -> i64 {
	let mut file = IndexedFile::from_str(input);
	file.mix();
	file.grove_coordinates()
}

fn get_answer_2_indexed(input: &str) -> i64 {
	let mut file = IndexedFile::from_str(input);
	file.multiply_numbers(DECRYPTION_KEY);
	for _ in 0..10 {
		file.mix();
	}
	file.grove_coordinates()
}

fn get_answer_1_ordered(input: &str) -> i64 {
	let mut file = OrderedFile::from_str(input);
	file.mix();
	file.grove_coordinates()
}

fn get_answer_2_ordered(input: &str) -> i64 {
	let mut file = OrderedFile::from_str(input);
	file.multiply_numbers(DECRYPTION_KEY);
	for _ in 0..10 {
		file.mix();
	}
	file.grove_coordinates()
}

const DECRYPTION_KEY: i64 = 811_589_153;

struct OrderedFile {
	numbers: Vec<(usize, i64)>,
}

impl OrderedFile {
	fn from_str(str: &str) -> Self {
		let numbers: Vec<(usize, i64)> = str
			.lines()
			.enumerate()
			.map(|(index, line)| (index, line.parse().unwrap()))
			.collect();
		Self { numbers }
	}
	fn multiply_numbers(&mut self, multiplier: i64) {
		for (_, value) in &mut self.numbers {
			*value *= multiplier;
		}
	}
	fn mix(&mut self) {
		for i in 0..self.numbers.len() {
			let (current_position, &(index, value)) = self
				.numbers
				.iter()
				.enumerate()
				.find(|(_, (index, _))| *index == i)
				.unwrap();
			self.numbers.remove(current_position);
			let new_position =
				(current_position as i64 + value).rem_euclid(self.numbers.len() as i64) as usize;
			self.numbers.insert(new_position, (index, value));
		}
	}
	fn grove_coordinates(&self) -> i64 {
		let zero = self
			.numbers
			.iter()
			.position(|(_, value)| *value == 0)
			.unwrap();
		(1..4)
			.map(|i| (zero + i * 1000) % self.numbers.len())
			.map(|i| self.numbers[i].1)
			.sum()
	}
}

struct IndexedFile {
	numbers: Vec<(usize, i64)>,
}

impl IndexedFile {
	fn from_str(str: &str) -> Self {
		let numbers: Vec<(usize, i64)> = str
			.lines()
			.enumerate()
			.map(|(index, line)| (index, line.parse().unwrap()))
			.collect();
		Self { numbers }
	}
	fn multiply_numbers(&mut self, multiplier: i64) {
		for (_, value) in &mut self.numbers {
			*value *= multiplier;
		}
	}
	fn mix(&mut self) {
		for n in 0..self.numbers.len() {
			let (position, value) = self.numbers[n];
			let new_position =
				(position as i64 + value).rem_euclid(self.numbers.len() as i64 - 1) as usize;
			if new_position > position {
				for (pos, _) in self.numbers.iter_mut() {
					if (position + 1..new_position + 1).contains(&*pos) {
						*pos -= 1
					}
				}
			} else {
				for (pos, _) in self.numbers.iter_mut() {
					if (new_position..position).contains(&*pos) {
						*pos += 1
					}
				}
			}
			self.numbers[n].0 = new_position;
		}
	}
	fn grove_coordinates(mut self) -> i64 {
		self.numbers.sort_by_key(|(position, _)| *position);
		let zero = self
			.numbers
			.iter()
			.position(|(_, value)| *value == 0)
			.unwrap();
		(1..4)
			.map(|n| {
				let index = (zero + n * 1000) % self.numbers.len();
				self.numbers[index].1
			})
			.sum()
	}
}

struct DoublyLinkedFile {
	numbers: Vec<i64>,
	links: Vec<DoublyLinkedItem>,
}

impl DoublyLinkedFile {
	fn from_str(str: &str) -> Self {
		let numbers: Vec<i64> = str.lines().map(|line| line.parse().unwrap()).collect();
		let links = make_doubly_linked_list(numbers.len());
		Self { numbers, links }
	}
	fn multiply_numbers(&mut self, multiplier: i64) {
		for number in &mut self.numbers {
			*number *= multiplier;
		}
	}
	fn mix(&mut self) {
		for (index, &num) in self.numbers.iter().enumerate() {
			let original_link = self.links[index];
			let mut link = original_link;
			let move_forward = num % (self.numbers.len() as i64 - 1);
			match move_forward {
				1.. => {
					for _ in 0..move_forward {
						link = self.links[link.next];
					}
				}
				..=-1 => {
					for _ in move_forward..1 {
						link = self.links[link.previous];
					}
				}
				0 => continue,
			}
			let link_after = link.next;
			let link_before = self.links[link_after].previous;
			self.links[link_before].next = index;
			self.links[link_after].previous = index;
			self.links[index].previous = link_before;
			self.links[index].next = link_after;
			self.links[original_link.next].previous = original_link.previous;
			self.links[original_link.previous].next = original_link.next;
		}
	}
	fn grove_coordinates(&self) -> i64 {
		let zero = self.numbers.iter().position(|&n| n == 0).unwrap();
		let mut link = self.links[zero];
		(0..3)
			.map(|_| {
				for _ in 0..1000 - 1 {
					link = self.links[link.next];
				}
				let number = self.numbers[link.next];
				link = self.links[link.next];
				number
			})
			.sum()
	}
}

#[derive(Clone, Copy)]
struct DoublyLinkedItem {
	previous: usize,
	next: usize,
}

fn make_doubly_linked_list(size: usize) -> Vec<DoublyLinkedItem> {
	[DoublyLinkedItem {
		previous: size - 1,
		next: 1,
	}]
	.into_iter()
	.chain((1..size - 1).map(|n| DoublyLinkedItem {
		previous: n - 1,
		next: n + 1,
	}))
	.chain([DoublyLinkedItem {
		previous: size - 2,
		next: 0,
	}])
	.collect()
}

struct SinglyLinkedFile {
	numbers: Vec<i64>,
	links: Vec<SinglyLinkedItem>,
}

impl SinglyLinkedFile {
	fn from_str(str: &str) -> Self {
		let numbers: Vec<i64> = str.lines().map(|line| line.parse().unwrap()).collect();
		let links = make_singly_linked_list(numbers.len());
		Self { numbers, links }
	}
	fn multiply_numbers(&mut self, multiplier: i64) {
		for number in &mut self.numbers {
			*number *= multiplier;
		}
	}
	fn mix(&mut self) {
		for (index, &num) in self.numbers.iter().enumerate() {
			let original_link = self.links[index];
			let old_link_after = original_link.next;
			let mut link = original_link;
			let move_forward = num.rem_euclid(self.numbers.len() as i64 - 1);
			if move_forward == 0 {
				continue;
			}
			for _ in 0..move_forward - 1 {
				link = self.links[link.next];
			}
			let new_link_before = link.next;
			for _ in move_forward - 1..self.numbers.len() as i64 - 2 {
				link = self.links[link.next];
			}
			let old_link_before = link.next;
			let new_link_after = self.links[new_link_before].next;
			self.links[new_link_before].next = index;
			self.links[index].next = new_link_after;
			self.links[old_link_before].next = old_link_after;
		}
	}
	fn grove_coordinates(&self) -> i64 {
		let zero = self.numbers.iter().position(|&n| n == 0).unwrap();
		let mut link = self.links[zero];
		(0..3)
			.map(|_| {
				for _ in 0..1000 - 1 {
					link = self.links[link.next];
				}
				let number = self.numbers[link.next];
				link = self.links[link.next];
				number
			})
			.sum()
	}
}

#[derive(Clone, Copy)]
struct SinglyLinkedItem {
	next: usize,
}

fn make_singly_linked_list(size: usize) -> Vec<SinglyLinkedItem> {
	(0..size - 1)
		.map(|n| SinglyLinkedItem { next: n + 1 })
		.chain([SinglyLinkedItem { next: 0 }])
		.collect()
}
