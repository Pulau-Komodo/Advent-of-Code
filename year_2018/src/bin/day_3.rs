use shared::{Grid, IntoCartesianProduct, Vec2};

fn main() {
	shared::print_answers(3, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> usize {
	let mut grid = Grid::empty(1000, 1000, 0);
	for claim in input.lines().map(Claim::from_line) {
		claim.apply(&mut grid);
	}
	grid.iter().filter(|claims| **claims > 1).count()
}

fn get_answer_2(input: &str) -> usize {
	let claims: Vec<_> = input.lines().map(Claim::from_line).collect();
	let mut grid = Grid::empty(1000, 1000, 0);
	for claim in &claims {
		claim.apply(&mut grid);
	}
	for (index, claim) in claims.into_iter().enumerate() {
		if !claim.check_overlap(&grid) {
			return index + 1;
		}
	}
	panic!();
}

struct Claim {
	position: Vec2<usize>,
	size: Vec2<usize>,
}

impl Claim {
	fn from_line(line: &str) -> Self {
		let (_, info) = line.split_once(" @ ").unwrap();
		let (position, size) = info.split_once(": ").unwrap();
		let (x, y) = position.split_once(',').unwrap();
		let position = Vec2::new(x.parse().unwrap(), y.parse().unwrap());
		let (x, y) = size.split_once('x').unwrap();
		let size = Vec2::new(x.parse().unwrap(), y.parse().unwrap());
		Self { position, size }
	}
	fn apply(&self, grid: &mut Grid<u16>) {
		for (x, y) in (self.position.x..self.position.x + self.size.x)
			.cartesian_product(self.position.y..self.position.y + self.size.y)
		{
			*grid.get_vec2_mut(Vec2::new(x, y)) += 1;
		}
	}
	fn check_overlap(&self, grid: &Grid<u16>) -> bool {
		(self.position.x..self.position.x + self.size.x)
			.cartesian_product(self.position.y..self.position.y + self.size.y)
			.any(|(x, y)| grid.get_vec2(Vec2::new(x, y)) > 1)
	}
}
