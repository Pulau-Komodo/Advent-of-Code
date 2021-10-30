fn main() {
	shared::print_answers(4, &[get_answers]);
}

fn get_answers(input: &str) -> String {
	let now = std::time::Instant::now();
	let mut first = None;
	let mut first_time = None;
	let mut second = None;
	for n in 0.. {
		let text = format!("{}{}", input, n);
		let hash = md5(text.as_bytes());
		if hash < THRESHOLD_1 && first.is_none() {
			first = Some(n);
			first_time = Some(now.elapsed().as_micros());
		}
		if hash < THRESHOLD_2 {
			second = Some(n);
			break;
		}
	}
	format!(
		"1: {} ({} μs), 2: {}",
		first.unwrap(),
		first_time.unwrap(),
		second.unwrap()
	)
}

const THRESHOLD_1: u128 = 0x0000_1000_0000_0000_0000_0000_0000_0000;
const THRESHOLD_2: u128 = 0x0000_0100_0000_0000_0000_0000_0000_0000;

const SHIFT: [u8; 64] = [
	7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9,
	14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10, 15,
	21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
];
const CONSTANTS: [u32; 64] = [
	0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
	0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
	0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
	0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed, 0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
	0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
	0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
	0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
	0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
];

const A: u32 = 0x67452301;
const B: u32 = 0xefcdab89;
const C: u32 = 0x98badcfe;
const D: u32 = 0x10325476;

#[allow(clippy::many_single_char_names)]
fn md5(data: &[u8]) -> u128 {
	let original_length = data.len() as u64 * 8; // % (2.pow(64));
	let mut data = data.to_vec();
	data.push(0x80);
	let needed_padding = 64 - (data.len() + 8) % 64;
	data.reserve_exact(needed_padding + 8);
	data.resize(data.len() + needed_padding, 0);
	data.extend_from_slice(&original_length.to_le_bytes());
	let (mut a, mut b, mut c, mut d) = (A, B, C, D);
	for chunk in data.chunks_exact(64) {
		let words: Vec<u32> = chunk
			.chunks_exact(4)
			.map(|word| u32::from_le_bytes([word[0], word[1], word[2], word[3]]))
			.collect();
		let (mut a2, mut b2, mut c2, mut d2) = (a, b, c, d);
		for i in 0..64 {
			let (mut f, g) = if i < 16 {
				(b2 & c2 | !b2 & d2, i)
			} else if i < 32 {
				(d2 & b2 | !d2 & c2, (5 * i + 1) % 16)
			} else if i < 48 {
				(b2 ^ c2 ^ d2, (3 * i + 5) % 16)
			} else {
				(c2 ^ (b2 | !d2), (7 * i) % 16)
			};
			f = f
				.wrapping_add(a2)
				.wrapping_add(CONSTANTS[i])
				.wrapping_add(words[g]);
			a2 = d2;
			d2 = c2;
			c2 = b2;
			b2 = b2.wrapping_add(f.rotate_left(SHIFT[i] as u32));
		}
		a = a.wrapping_add(a2);
		b = b.wrapping_add(b2);
		c = c.wrapping_add(c2);
		d = d.wrapping_add(d2);
	}
	let a: [u8; 4] = a.to_le_bytes();
	let b: [u8; 4] = b.to_le_bytes();
	let c: [u8; 4] = c.to_le_bytes();
	let d: [u8; 4] = d.to_le_bytes();
	u128::from_be_bytes([
		a[0], a[1], a[2], a[3], b[0], b[1], b[2], b[3], c[0], c[1], c[2], c[3], d[0], d[1], d[2],
		d[3],
	])
}

#[cfg(test)]
mod tests {
	use super::*;

	fn test_hash(input: &str, expected: u128) {
		let hash = md5(input.as_bytes());
		println!("{:0128b}", hash);
		println!("{:0128b}", expected);
		println!("{:032x}", hash);
		println!("{:032x}", expected);
		assert_eq!(hash, expected);
	}

	#[test]
	fn empty_input() {
		test_hash("", 0xd41d8cd98f00b204e9800998ecf8427e);
	}

	#[test]
	fn quick_brown_dog() {
		test_hash(
			"The quick brown fox jumps over the lazy dog",
			0x9e107d9d372bb6826bd81d3542a419d6,
		);
	}

	#[test]
	fn long() {
		test_hash(
			"This is a longer message intended to test whether the hashing still works if the data spreads across multiple chunks.",
			0x1151cc2370bb985a7827614546c1047c,
		);
	}

	#[test]
	fn test_be_le() {
		println!(
			"{}, {}",
			u32::from_be_bytes([5, 20, 250, 100]),
			u32::from_le_bytes([100, 250, 20, 5])
		);
		println!(
			"{}",
			((5 as u32) << 24) + ((20 as u32) << 16) + ((250 as u32) << 8) + 100 as u32
		);
		const NUMBER: u32 = 0x10325476;
		let reversed = NUMBER.to_le_bytes();
		println!(
			"{:08x}, {:08x}",
			NUMBER,
			u32::from_be_bytes([reversed[0], reversed[1], reversed[2], reversed[3]])
		);
	}
}
