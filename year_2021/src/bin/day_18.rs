fn main() {
	shared::print_answers(18, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	let elements = input.lines().map(Element::from_str);
	let element = elements
		.reduce(|prev, curr| {
			let mut element = Element::pair(prev, curr);
			element.reduce();
			element
		})
		.unwrap();
	element.magnitude()
}

fn get_answer_2(input: &str) -> u32 {
	let elements: Vec<_> = input.lines().map(Element::from_str).collect();
	let mut max_magnitude = 0;
	for a in 0..elements.len() {
		for b in 0..elements.len() {
			if a == b {
				continue;
			}
			let mut element = Element::pair(elements[a].clone(), elements[b].clone());
			element.reduce();
			let magnitude = element.magnitude();
			max_magnitude = max_magnitude.max(magnitude);
		}
	}
	max_magnitude
}

enum Exploded {
	Yes(u8, u8),
	No,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Element {
	Pair(Box<Element>, Box<Element>),
	Number(u8),
}

impl Element {
	fn from_str(str: &str) -> Self {
		let bytes = str.as_bytes();
		if bytes[0] == b'[' {
			let split_pos = (|| {
				let mut open_brackets = 0;
				for (index, byte) in bytes.iter().enumerate() {
					match byte {
						b'[' => open_brackets += 1,
						b']' => open_brackets -= 1,
						b',' if open_brackets == 1 => return index,
						_ => (),
					}
				}
				panic!("Pair comma not found");
			})();
			let left = Element::from_str(&str[1..split_pos]);
			let right = Element::from_str(&str[split_pos + 1..str.len() - 1]);
			Self::Pair(Box::new(left), Box::new(right))
		} else {
			Self::Number(str.parse().unwrap())
		}
	}
	fn from_two_numbers(left: u8, right: u8) -> Self {
		Self::Pair(Box::new(Self::Number(left)), Box::new(Self::Number(right)))
	}
	fn pair(left: Self, right: Self) -> Self {
		Self::Pair(Box::new(left), Box::new(right))
	}
	fn reduce(&mut self) {
		loop {
			if let Exploded::No = self.explode(0) {
				if !self.split() {
					break;
				}
			}
		}
	}
	fn explode(&mut self, depth: u8) -> Exploded {
		match self {
			Self::Pair(left, right) => {
				if depth == 4 {
					if let (&Self::Number(left), &Self::Number(right)) = (&**left, &**right) {
						*self = Self::Number(0);
						Exploded::Yes(left, right)
					} else {
						println!("{:?}", self);
						panic!();
					}
				} else if let Exploded::Yes(add_left, add_right) = left.explode(depth + 1) {
					right.add_leftmost(add_right);
					Exploded::Yes(add_left, 0)
				} else if let Exploded::Yes(add_left, add_right) = right.explode(depth + 1) {
					left.add_rightmost(add_left);
					Exploded::Yes(0, add_right)
				} else {
					Exploded::No
				}
			}
			Self::Number(_) => Exploded::No,
		}
	}
	fn add_leftmost(&mut self, value: u8) {
		if value > 0 {
			match self {
				Self::Pair(left, _) => left.add_leftmost(value),
				Self::Number(n) => *n += value,
			}
		}
	}
	fn add_rightmost(&mut self, value: u8) {
		if value > 0 {
			match self {
				Self::Pair(_, right) => right.add_rightmost(value),
				Self::Number(n) => *n += value,
			}
		}
	}
	/// Returns whether any change occurred
	fn split(&mut self) -> bool {
		match self {
			Self::Pair(left, right) => left.split() || right.split(),
			&mut Self::Number(n) if n >= 10 => {
				let left = n / 2;
				let right = n - left;
				*self = Self::from_two_numbers(left, right);
				true
			}
			_ => false,
		}
	}
	fn magnitude(&self) -> u32 {
		match self {
			Self::Pair(left, right) => 3 * left.magnitude() + 2 * right.magnitude(),
			Self::Number(n) => *n as u32,
		}
	}
}
