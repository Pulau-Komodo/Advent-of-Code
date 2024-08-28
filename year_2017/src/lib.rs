use std::fmt::Display;

const LIST_SIZE: usize = 256;

pub struct KnotHash([u8; 16]);

impl KnotHash {
	pub fn new(input: &str) -> Self {
		let suffix = [17, 31, 73, 47, 23];

		let mut list: [u8; LIST_SIZE] = std::array::from_fn(|i| i as u8);
		let mut position = 0;
		let mut skip_size = 0;
		for _ in 0..64 {
			for length in input.bytes().chain(suffix).map(|n| n as usize) {
				for offset in 0..length / 2 {
					swap(&mut list, position + offset, position + length - offset - 1);
				}
				position += length + skip_size;
				position %= LIST_SIZE;
				skip_size += 1;
			}
		}
		let hash = std::array::from_fn(|i| {
			let offset = i * 16;
			xor_slice(&list[offset..offset + 16])
		});
		Self(hash)
	}
	pub fn count_ones(&self) -> u32 {
		self.0.iter().map(|n| n.count_ones()).sum()
	}
	pub fn ones_iter(&self) -> impl DoubleEndedIterator<Item = bool> + '_ {
		self.0.iter().copied().rev().flat_map(|mut n| {
			(0..8).map(move |_| {
				let output = n % 2 == 1;
				n /= 2;
				output
			})
		})
	}
}

impl Display for KnotHash {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for byte in self.0 {
			f.write_fmt(format_args!("{:02x}", byte))?
		}
		Ok(())
	}
}

fn swap(list: &mut [u8; LIST_SIZE], a: usize, b: usize) {
	list.swap(a % LIST_SIZE, b % LIST_SIZE);
}

fn xor_slice(slice: &[u8]) -> u8 {
	slice.iter().copied().reduce(|acc, n| acc ^ n).unwrap()
}
