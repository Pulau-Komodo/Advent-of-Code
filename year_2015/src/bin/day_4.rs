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
		let hash = shared::md5(text.as_bytes());
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
