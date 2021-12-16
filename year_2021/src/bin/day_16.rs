fn main() {
	shared::print_answers(16, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u64 {
	let mut message = Message::from_str(input);
	let packet = message.next_packet();
	packet.version_sum() as u64
}

fn get_answer_2(input: &str) -> u64 {
	let mut message = Message::from_str(input);
	let packet = message.next_packet();
	packet.evaluate()
}

#[derive(Debug)]
enum Packet {
	Value {
		version: u8,
		value: u64,
	},
	Operator {
		version: u8,
		type_id: u8,
		contents: Vec<Packet>,
	},
}

impl Packet {
	fn version_sum(&self) -> u32 {
		match self {
			Self::Value { version, .. } => *version as u32,
			Self::Operator {
				version, contents, ..
			} => contents.iter().map(Self::version_sum).sum::<u32>() + *version as u32,
		}
	}
	fn evaluate(&self) -> u64 {
		match self {
			Self::Value { value, .. } => *value,
			Self::Operator {
				type_id, contents, ..
			} => {
				let mut evaluated = contents.iter().map(|packet| packet.evaluate());
				match type_id {
					0 => evaluated.sum(),
					1 => evaluated.product(),
					2 => evaluated.min().unwrap(),
					3 => evaluated.max().unwrap(),
					5 => (evaluated.next().unwrap() > evaluated.next().unwrap()) as u64,
					6 => (evaluated.next().unwrap() < evaluated.next().unwrap()) as u64,
					7 => (evaluated.next().unwrap() == evaluated.next().unwrap()) as u64,
					_ => panic!("Invalid type ID"),
				}
			}
		}
	}
}

struct Message {
	bits: Vec<bool>,
	position: usize,
}

impl Message {
	fn from_str(str: &str) -> Self {
		let bits = str
			.chars()
			.flat_map(|char| {
				let byte = char_to_byte(char);
				(0_u8..4_u8).rev().map(move |i| 1 << i & byte != 0)
			})
			.collect();
		Self { bits, position: 0 }
	}
	fn next_packet(&mut self) -> Packet {
		let version = u8_from_bits(self.read_n(3));
		let type_id = u8_from_bits(self.read_n(3));
		if type_id == 4 {
			let mut value = 0;
			let mut has_more_groups = true;
			while has_more_groups {
				has_more_groups = self.read_one();
				value <<= 4;
				value |= u8_from_bits(self.read_n(4)) as u64;
			}
			Packet::Value { version, value }
		} else {
			let length_type = self.read_one();
			let contents = if length_type {
				let length = u16_from_bits(self.read_n(11));
				self.n_packets(length)
			} else {
				let length = u16_from_bits(self.read_n(15));
				self.packets_from_n_bits(length)
			};
			Packet::Operator {
				version,
				type_id,
				contents,
			}
		}
	}
	fn n_packets(&mut self, n: usize) -> Vec<Packet> {
		(0..n).map(|_| self.next_packet()).collect()
	}
	fn packets_from_n_bits(&mut self, n: usize) -> Vec<Packet> {
		let end_pos = self.position + n;
		let mut output = Vec::new();
		while self.position < end_pos {
			output.push(self.next_packet());
		}
		output
	}
	fn read_n(&mut self, n: usize) -> &[bool] {
		let start = self.position;
		self.position += n;
		&self.bits[start..self.position]
	}
	fn read_one(&mut self) -> bool {
		let bit = self.bits[self.position];
		self.position += 1;
		bit
	}
}

fn char_to_byte(char: char) -> u8 {
	match char {
		'0'..='9' => char as u8 - 48,
		'A'..='F' => char as u8 - 55,
		_ => panic!(),
	}
}

fn u8_from_bits(bits: &[bool]) -> u8 {
	let mut number = 0;
	for &bit in bits {
		number <<= 1;
		number |= bit as u8;
	}
	number
}

fn u16_from_bits(bits: &[bool]) -> usize {
	let mut number = 0;
	for &bit in bits {
		number <<= 1;
		number |= bit as usize;
	}
	number
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn sample_input_1() {
		assert_eq!(16, get_answer_1("8A004A801A8002F478"));
		assert_eq!(12, get_answer_1("620080001611562C8802118E34"));
		assert_eq!(23, get_answer_1("C0015000016115A2E0802F182340"));
		assert_eq!(31, get_answer_1("A0016C880162017C3686B18A3D4780"));
	}

	#[test]
	fn sample_input_2() {
		assert_eq!(3, get_answer_2("C200B40A82"));
		assert_eq!(54, get_answer_2("04005AC33890"));
		assert_eq!(7, get_answer_2("880086C3E88112"));
		assert_eq!(9, get_answer_2("CE00C43D881120"));
		assert_eq!(1, get_answer_2("D8005AC2A8F0"));
		assert_eq!(0, get_answer_2("F600BC2D8F"));
		assert_eq!(0, get_answer_2("9C005AC2F8F0"));
		assert_eq!(1, get_answer_2("9C0141080250320F1802104A08"));
	}

	#[test]
	fn test_bittification() {
		let input = "8A004A801A8002F478";
		let message = Message::from_str(input);
		assert_eq!(message.bits.len(), input.len() * 4);
	}
}
