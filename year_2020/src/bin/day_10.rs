fn main() {
	shared::print_answers(10, &[get_answers]);
}

fn get_answers(input: &str) -> String {
	let mut adapters = input
		.lines()
		.map(str::parse::<u16>)
		.collect::<Result<Vec<_>, _>>()
		.unwrap();
	adapters.sort_unstable();
	let mut previous_number = 0;
	let mut one_jolt_count = 0;
	let mut three_jolt_count = 1;
	let mut one_jolt_cluster_size = 0;
	let mut possibilities: u64 = 1;
	for number in adapters {
		let difference = number - previous_number;
		previous_number = number;
		if difference == 1 {
			one_jolt_count += 1;
			one_jolt_cluster_size += 1;
		} else if difference == 3 {
			three_jolt_count += 1;
			possibilities *= calculate_possibilities(one_jolt_cluster_size);
			one_jolt_cluster_size = 0;
		}
	}
	let product = one_jolt_count * three_jolt_count;
	format!("1: {}, 2: {}", product, possibilities)
}

fn calculate_possibilities(one_jolt_cluster_size: u16) -> u64 {
	match one_jolt_cluster_size {
		0..=1 => 1,
		2 => 2,
		3 => 4,
		4 => 7,
		_ => unreachable!(),
	}
}
