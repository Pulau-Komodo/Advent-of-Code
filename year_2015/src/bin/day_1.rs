fn main() {
	shared::print_answers(1, &[get_answers]);
}

fn get_answers(input: &str) -> String {
	let mut floor = 0;
	let mut first_basement = 0;
	for (index, change) in input.chars().map(|char| match char {
		'(' => 1,
		')' => -1,
		_ => panic!(),
	}).enumerate() {
		floor += change;
		if first_basement == 0 && floor == -1 {
			first_basement = index + 1;
		}
	}
	format!("1: {}, 2: {}", floor, first_basement)
}