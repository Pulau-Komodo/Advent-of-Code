use std::collections::HashSet;

fn main() {
	shared::print_answers(20, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	let (key, image) = input.split_once("\r\n\r\n").unwrap();
	let key = parse_key(key);
	let mut image = Image::from_str(image);
	for _ in 0..2 {
		image.enhance(&key);
	}
	image.light_pixels.len() as u32
}

fn get_answer_2(input: &str) -> u32 {
	let (key, image) = input.split_once("\r\n\r\n").unwrap();
	let key = parse_key(key);
	let mut image = Image::from_str(image);
	for _ in 0..50 {
		image.enhance(&key);
	}
	image.light_pixels.len() as u32
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
	x: i16,
	y: i16,
}

struct Image {
	light_pixels: HashSet<Point>,
	inverted: bool,
	x_start: i16,
	x_end: i16,
	y_start: i16,
	y_end: i16,
}

impl Image {
	fn from_str(str: &str) -> Self {
		let light_pixels = str
			.lines()
			.enumerate()
			.flat_map(|(y, line)| {
				line.as_bytes()
					.iter()
					.map(move |byte| (y, byte))
					.enumerate()
			})
			.filter(|(_, (_, &byte))| byte == b'#')
			.map(|(x, (y, _))| Point {
				x: x as i16,
				y: y as i16,
			})
			.collect();
		Self {
			light_pixels,
			inverted: false,
			x_start: 0,
			x_end: 100,
			y_start: 0,
			y_end: 100,
		}
	}
	fn window_around(&self, point: &Point) -> usize {
		let mut value = 0;
		for (x_offset, y_offset) in (-1..=1).flat_map(|y| (-1..=1).map(move |x| (x, y))) {
			value <<= 1;
			let offset_point = Point {
				x: point.x + x_offset,
				y: point.y + y_offset,
			};
			if self.inverted ^ self.light_pixels.contains(&offset_point) {
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
		self.light_pixels = (self.y_start..self.y_end)
			.flat_map(|y| (self.x_start..self.x_end).map(move |x| Point { x, y }))
			.filter(|point| !self.inverted ^ key[self.window_around(point)])
			.collect();
		self.inverted = !self.inverted;
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_window() {
		let image = Image::from_str("###\n..#\n##.");
		let window = image.window_around(&Point { x: 1, y: 1 });
		//println!("{:09b}", window);
		assert_eq!(0b111001110, window);
	}
}
