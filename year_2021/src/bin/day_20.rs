fn main() {
	shared::print_answers(20, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> usize {
	let (key, image) = input.split_once("\r\n\r\n").unwrap();
	let key = parse_key(key);
	const ITERATIONS: usize = 2;
	let mut image = Image::<{ 100 + 2 + ITERATIONS * 2 }>::from_str(image, 100);
	for _ in 0..ITERATIONS {
		image.enhance(&key);
	}
	image.count_light()
}

fn get_answer_2(input: &str) -> usize {
	let (key, image) = input.split_once("\r\n\r\n").unwrap();
	let key = parse_key(key);
	const ITERATIONS: usize = 50;
	let mut image = Image::<{ 100 + 2 + ITERATIONS * 2 }>::from_str(image, 100);
	for _ in 0..ITERATIONS {
		image.enhance(&key);
	}
	image.count_light()
}

fn parse_key(str: &str) -> [bool; 512] {
	let mut key = [false; 512];
	for (index, &byte) in str.as_bytes().iter().enumerate() {
		if byte == b'#' {
			key[index] = true;
		}
	}
	key
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
	x: usize,
	y: usize,
}

struct Image<const SIZE: usize> {
	light_pixels: Box<[[bool; SIZE]; SIZE]>,
	inverted: bool,
	x_start: usize,
	x_end: usize,
	y_start: usize,
	y_end: usize,
}

impl<const SIZE: usize> Image<SIZE> {
	fn from_str(str: &str, starting_size: usize) -> Self {
		let padding = (SIZE - starting_size) / 2;
		let mut light_pixels = Box::new([[false; SIZE]; SIZE]);
		str.lines()
			.enumerate()
			.flat_map(|(y, line)| {
				line.as_bytes()
					.iter()
					.map(move |byte| (y, byte))
					.enumerate()
			})
			.for_each(|(x, (y, &byte))| light_pixels[y + padding][x + padding] = byte == b'#');
		Self {
			light_pixels,
			inverted: false,
			x_start: padding,
			x_end: SIZE - padding,
			y_start: padding,
			y_end: SIZE - padding,
		}
	}
	fn get(&self, point: &Point) -> bool {
		self.light_pixels[point.y][point.x]
	}
	fn window_around(&self, point: &Point) -> usize {
		let mut value = 0;
		for (x_offset, y_offset) in (0..=2).flat_map(|y| (0..=2).map(move |x| (x, y))) {
			value <<= 1;
			let offset_point = Point {
				x: point.x + x_offset - 1,
				y: point.y + y_offset - 1,
			};
			if self.inverted ^ self.get(&offset_point) {
				value |= 1;
			}
		}
		value
	}
	fn enhance(&mut self, key: &[bool; 512]) {
		self.x_start -= 1;
		self.x_end += 1;
		self.y_start -= 1;
		self.y_end += 1;
		let mut new_pixels = Box::new([[false; SIZE]; SIZE]);
		(self.y_start..self.y_end)
			.flat_map(|y| (self.x_start..self.x_end).map(move |x| Point { x, y }))
			.for_each(|point| {
				new_pixels[point.y][point.x] = !self.inverted ^ key[self.window_around(&point)]
			});
		self.light_pixels = new_pixels;
		self.inverted = !self.inverted;
	}
	fn count_light(&self) -> usize {
		self.light_pixels
			.iter()
			.flatten()
			.filter(|&&light| light)
			.count()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_window() {
		let image = Image::<5>::from_str("###\n..#\n##.", 3);
		let window = image.window_around(&Point { x: 1, y: 1 });
		println!("{:09b}", window);
		assert_eq!(0b111001110, window);
	}
}
