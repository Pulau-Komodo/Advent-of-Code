fn main() {
	shared::print_answers(16, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> String {
	let data = Vec::from(String::from(input).as_bytes());
	let padded_data = pad_data(data, TARGET_LENGTH_1);
	let checksum = checksum(padded_data);
	checksum.into_iter().map(char::from).collect()
}

fn get_answer_2(input: &str) -> String {
	let data = Vec::from(String::from(input).as_bytes());
	let padded_data = pad_data(data, TARGET_LENGTH_2);
	let checksum = checksum(padded_data);
	checksum.into_iter().map(char::from).collect()
}

const TARGET_LENGTH_1: usize = 272;
const TARGET_LENGTH_2: usize = 35651584;

fn pad_data(mut data: Vec<u8>, target_length: usize) -> Vec<u8> {
	while data.len() < target_length {
		data.push(b'0');
		for n in (0..data.len() - 1).rev() {
			let new_datum = if data[n] == b'0' { b'1' } else { b'0' };
			data.push(new_datum);
		}
	}
	Vec::from(&data[0..target_length])
}

fn checksum(mut data: Vec<u8>) -> Vec<u8> {
	while data.len() % 2 == 0 {
		for i in 0..data.len() / 2 {
			let value = if data[i * 2] == data[i * 2 + 1] {
				b'1'
			} else {
				b'0'
			};
			data[i] = value;
		}
		data.drain(data.len() / 2..);
	}
	data
}
