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
pub fn md5(data: &[u8]) -> u128 {
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
	fn extra_long() {
		test_hash(
			"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Morbi mattis felis sem, ac tincidunt lectus vulputate eget. Nam viverra egestas erat vel accumsan. Nullam malesuada diam in interdum consectetur. Morbi mi mi, vulputate gravida est in, rhoncus elementum magna. In volutpat finibus dapibus. Sed ipsum ligula, condimentum at tempus id, porttitor quis urna. Sed sollicitudin eros a convallis molestie. Integer diam dolor, varius non neque ut, porttitor mattis est. Vestibulum a eros quis turpis blandit iaculis quis vitae neque. Maecenas suscipit ex ac mi venenatis, at sodales enim eleifend. Phasellus tristique dictum lacus nec facilisis. Suspendisse a auctor nibh, at bibendum lorem. Duis lobortis lobortis efficitur. Sed sagittis ex sit amet mi congue, at ornare ex accumsan. Maecenas eleifend elit semper, eleifend augue non, mollis neque. Morbi vitae interdum sem. Morbi maximus mi et condimentum porttitor. Pellentesque dictum nibh efficitur nibh dapibus posuere. Sed nec ultrices dolor. Nullam justo est, ultrices vitae mauris ultricies, vehicula condimentum nisi. Donec consectetur sit amet sapien ac rhoncus. In sed dictum nisi. Nulla faucibus id enim in lobortis. Mauris luctus sem nibh, sed venenatis diam faucibus ac. Duis rutrum lectus sed est egestas iaculis. Morbi lobortis, nisi eu semper dictum, est quam cursus tellus, vel consequat leo arcu id tellus. Ut egestas purus quis justo pharetra dignissim. Curabitur facilisis urna in ligula dictum rutrum. Maecenas non mauris neque. Suspendisse porta tellus eros, a facilisis lacus scelerisque ut. Aliquam eget libero nec elit pellentesque iaculis sed sit amet lectus. Sed eu orci semper, aliquet neque et, eleifend nibh. Sed volutpat sem rhoncus pharetra tempor. Aenean imperdiet est metus, a laoreet enim condimentum in. Praesent nisi felis, cursus id tempor quis, suscipit at augue. Pellentesque et dictum velit. Cras cursus vehicula dolor, vel euismod nibh tempus vitae. Nullam convallis nulla sed cursus cursus. In sollicitudin tortor lectus, vel feugiat magna sollicitudin in. Maecenas placerat consectetur mauris ac ultricies. Nulla luctus, sem at ultrices facilisis, justo tortor commodo urna, aliquam tincidunt neque leo fringilla massa. Suspendisse in aliquet augue, in vulputate urna. Maecenas dapibus varius sapien at fermentum. Suspendisse non metus ut ex rhoncus lacinia id ut ipsum. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Fusce porta posuere eros, sollicitudin sollicitudin diam eleifend gravida. Aliquam erat volutpat. Praesent ut magna egestas, fermentum sapien id, aliquam tellus. Curabitur dictum sem est, a rutrum est efficitur nec. Donec semper tristique diam, eget sodales tortor cursus vel. Suspendisse a tempor elit. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Morbi ullamcorper lorem a neque porttitor dapibus. Nam molestie lorem ut lacus pharetra, nec commodo leo pretium. Suspendisse hendrerit elementum iaculis. Praesent nec vehicula justo. Fusce iaculis id lorem sed faucibus. Nunc molestie leo eros, in dictum turpis elementum eu. Aliquam pellentesque ante dolor, non iaculis est sollicitudin eu. Nam interdum est lorem, id interdum enim suscipit sit amet. Fusce efficitur augue in mauris tempor, et iaculis leo ornare. Donec accumsan ultrices tristique. Aliquam in vehicula lorem. Vivamus ac pretium velit. Aliquam tempus, elit efficitur facilisis eleifend, eros ipsum ultrices mauris, vitae porta arcu dui dignissim ex. Aenean nisi magna, dapibus eget eros et, lobortis lacinia tortor. Etiam commodo mollis urna, vel facilisis tellus molestie quis. Donec eu tellus ut nisi consectetur consectetur. Nullam purus felis, rhoncus ac porta et, pellentesque in quam. Sed blandit justo quis arcu porta venenatis. Quisque eu orci tempus, bibendum metus vitae, suscipit risus. Fusce porta, elit eget congue convallis, erat eros tincidunt libero, nec feugiat elit est at augue. Maecenas accumsan, diam id pretium tincidunt, velit felis gravida nulla, in tincidunt dui risus vel tortor. Curabitur sollicitudin cursus elit nec congue. Proin blandit felis a lectus gravida, a accumsan mauris iaculis. Mauris quis luctus lectus. Vestibulum porta dui id risus feugiat, eu rutrum sapien consequat. Mauris urna justo, hendrerit eget lorem ut, malesuada commodo ligula. Aliquam nec mauris lectus.",
			0x39fb988c2e843debacf9e34b70b284cc,
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
