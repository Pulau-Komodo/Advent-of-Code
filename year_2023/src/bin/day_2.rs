fn main() {
	shared::print_answers(2, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	input
		.lines()
		.map(Record::from_line)
		.enumerate()
		.filter(|(_, record)| record.sets.iter().all(|set| set.fits_inside(&TEST_SET)))
		.map(|(index, _)| index as u32 + 1)
		.sum()
}

fn get_answer_2(input: &str) -> u32 {
	input
		.lines()
		.map(Record::from_line)
		.map(|record| {
			record
				.sets
				.iter()
				.fold(CubeSet::default(), |acc, el| CubeSet::max(&acc, el))
				.power()
		})
		.sum()
}

const TEST_SET: CubeSet = CubeSet {
	red: 12,
	green: 13,
	blue: 14,
};

#[derive(Default)]
struct CubeSet {
	red: u32,
	green: u32,
	blue: u32,
}

impl CubeSet {
	fn from_str(str: &str) -> Self {
		let mut set = CubeSet::default();
		for substring in str.split(", ") {
			let (count, colour) = substring.split_once(' ').unwrap();
			let count = count.parse().unwrap();
			match colour {
				"red" => set.red = count,
				"green" => set.green = count,
				"blue" => set.blue = count,
				_ => panic!("Invalid colour"),
			}
		}
		set
	}
	fn fits_inside(&self, other: &Self) -> bool {
		self.red <= other.red && self.green <= other.green && self.blue <= other.blue
	}
	fn max(&self, other: &Self) -> Self {
		Self {
			red: u32::max(self.red, other.red),
			green: u32::max(self.green, other.green),
			blue: u32::max(self.blue, other.blue),
		}
	}
	fn power(&self) -> u32 {
		self.red * self.green * self.blue
	}
}

struct Record {
	sets: Vec<CubeSet>,
}

impl Record {
	fn from_line(line: &str) -> Self {
		let (_game, sets) = line.split_once(": ").unwrap();
		let sets = sets.split("; ").map(CubeSet::from_str).collect();
		Self { sets }
	}
}
