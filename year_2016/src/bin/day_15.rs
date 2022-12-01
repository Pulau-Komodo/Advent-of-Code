fn main() {
	shared::print_answers(15, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	let discs: Vec<_> = input.lines().map(Disc::from_str).collect();
	find_first_aligned_moment(&discs)
}

fn get_answer_2(input: &str) -> u32 {
	let mut discs: Vec<_> = input.lines().map(Disc::from_str).collect();
	discs.push(Disc {
		starting_pos: 0,
		interval: 11,
	});
	find_first_aligned_moment(&discs)
}

struct Disc {
	starting_pos: u32,
	interval: u32,
}

impl Disc {
	fn from_str(str: &str) -> Self {
		let (first, last) = str
			.split_once(" positions; at time=0, it is at position ")
			.unwrap();
		let (_, interval) = first.rsplit_once(' ').unwrap();
		let (starting_pos, _) = last.rsplit_once('.').unwrap();
		let interval = interval.parse().unwrap();
		let starting_pos = starting_pos.parse().unwrap();
		Disc {
			starting_pos,
			interval,
		}
	}
}

fn find_first_aligned_moment(discs: &[Disc]) -> u32 {
	let mut cycle = 1;
	let mut starting_point = 0;
	for (
		index,
		Disc {
			starting_pos,
			interval,
		},
	) in discs.iter().enumerate()
	{
		let target = (index as u32 + 1) % interval;
		let target = 2 * interval - starting_pos - target % interval;
		let cycles = how_many_cycles_to_target(
			starting_point % interval,
			target % interval,
			cycle % interval,
			*interval,
		);
		starting_point += cycles * cycle;
		cycle *= interval;
	}
	starting_point
}

fn how_many_cycles_to_target(start: u32, target: u32, step: u32, modulo: u32) -> u32 {
	let mut cycles = 0;
	let mut current = start;
	loop {
		if current == target {
			break cycles;
		}
		cycles += 1;
		current = (current + step) % modulo;
	}
}
