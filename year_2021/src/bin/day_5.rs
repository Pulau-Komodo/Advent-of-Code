fn main() {
	shared::print_answers(5, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	let lines = parse_input(input, false);
	let vents = vent_map(lines);
	vents.iter().filter(|(_, &count)| count > 1).count() as u32
}

fn get_answer_2(input: &str) -> u32 {
	let lines = parse_input(input, true);
	let vents = vent_map(lines);
	vents.iter().filter(|(_, &count)| count > 1).count() as u32
}

struct Point {
	x: i16,
	y: i16,
}

type Line = (Point, Point);

fn parse_input(input: &str, include_diagonals: bool) -> impl Iterator<Item = Line> + '_ {
	input
		.lines()
		.map(|line| {
			let (start, end) = line.split_once(" -> ").unwrap();
			let (start_x, start_y) = start.split_once(',').unwrap();
			let (end_x, end_y) = end.split_once(',').unwrap();
			((start_x, start_y), (end_x, end_y))
		})
		.filter(move |((start_x, start_y), (end_x, end_y))| {
			include_diagonals || start_x == end_x || start_y == end_y
		})
		.map(|((start_x, start_y), (end_x, end_y))| {
			(
				Point {
					x: start_x.parse().unwrap(),
					y: start_y.parse().unwrap(),
				},
				Point {
					x: end_x.parse().unwrap(),
					y: end_y.parse().unwrap(),
				},
			)
		})
}

fn vent_map(lines: impl Iterator<Item = Line>) -> std::collections::HashMap<(i16, i16), u8> {
	let mut map = std::collections::HashMap::new();
	for (start, end) in lines {
		let (d_x, d_y) = (end.x - start.x, end.y - start.y);
		let steps = d_x.abs().max(d_y.abs());
		let (s_x, s_y) = (d_x.signum(), d_y.signum());
		for i in 0..=steps {
			let point = (start.x + i * s_x, start.y + i * s_y);
			let entry = map.entry(point).or_default();
			*entry += 1;
		}
	}
	map
}
