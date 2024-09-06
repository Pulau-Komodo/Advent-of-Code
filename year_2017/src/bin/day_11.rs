use shared::Vec3;

fn main() {
	shared::print_answers(11, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	let mut pos = Vec3::new(0, 0, 0);
	for step in input.trim().split(',') {
		let offset = match step {
			"n" => Vec3::new(0, 1, -1),
			"ne" => Vec3::new(1, 0, -1),
			"se" => Vec3::new(1, -1, 0),
			"s" => Vec3::new(0, -1, 1),
			"sw" => Vec3::new(-1, 0, 1),
			"nw" => Vec3::new(-1, 1, 0),
			_ => panic!(),
		};
		pos += offset;
	}
	[pos.x, pos.y, pos.z]
		.into_iter()
		.map(i32::unsigned_abs)
		.max()
		.unwrap()
}

fn get_answer_2(input: &str) -> u32 {
	let mut max = 0;
	let mut pos = Vec3::new(0, 0, 0);
	for step in input.trim().split(',') {
		let offset = match step {
			"n" => Vec3::new(0, 1, -1),
			"ne" => Vec3::new(1, 0, -1),
			"se" => Vec3::new(1, -1, 0),
			"s" => Vec3::new(0, -1, 1),
			"sw" => Vec3::new(-1, 0, 1),
			"nw" => Vec3::new(-1, 1, 0),
			_ => panic!(),
		};
		pos += offset;
		for value in [pos.x, pos.y, pos.z].into_iter().map(i32::unsigned_abs) {
			max = max.max(value);
		}
	}
	max
}
