use std::io::stdin;

use shared::{Grid, Offset, Point};

fn main() {
	shared::print_answers(14, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> i32 {
	let mut quadrants = [0; 4];
	for quadrant in input
		.lines()
		.map(Robot::from_line)
		.map(|robot| robot.progress(100))
		.filter_map(quadrant)
	{
		quadrants[quadrant] += 1;
	}
	quadrants.into_iter().product()
}

fn get_answer_2(input: &str) -> i32 {
	let mut robots = input.lines().map(Robot::from_line).collect::<Vec<_>>();
	let mut i = 0;
	loop {
		let mut grid = Grid::empty(WIDTH as usize, HEIGHT as usize, ' ');
		for robot in &robots {
			*grid.get_point_mut(Point::new(
				robot.position.x as usize,
				robot.position.y as usize,
			)) = '█';
		}
		grid.print_with(|char| *char);
		println!("{i}: Does this resemble a Christmas tree? y to confirm, a number to progress that many steps.");
		// Ending up finding a suspicious band at 27, recurring every 103 (`HEIGHT`) steps, and a suspicious band at 52, recurring every 101 (`WIDTH`) steps. Chinese remainder theorem would show when these would coincide, but I just calculated a bunch of values ahead on a spreadsheet and entered that.
		let mut input = String::new();
		stdin().read_line(&mut input).unwrap();
		let step_size = if input.trim() == "y" {
			return i;
		} else {
			input.trim().parse::<i32>().unwrap_or(1)
		};
		for robot in &mut robots {
			robot.position = robot.progress(step_size);
		}
		i += step_size;
	}
}

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

struct Robot {
	position: Point<i32>,
	velocity: Offset<i32>,
}

impl Robot {
	fn from_line(line: &str) -> Self {
		let (start, velocity) = line[2..].split_once(" v=").unwrap();
		let start = Point::from_comma_separated(start);
		let velocity = Point::from_comma_separated(velocity) - Point::zero();
		Self {
			position: start,
			velocity,
		}
	}
	fn progress(&self, steps: i32) -> Point<i32> {
		let point = self.position + self.velocity * steps;
		Point::new(point.x.rem_euclid(WIDTH), point.y.rem_euclid(HEIGHT))
	}
}

fn quadrant(point: Point<i32>) -> Option<usize> {
	use std::cmp::Ordering::*;
	match (point.x.cmp(&(WIDTH / 2)), point.y.cmp(&(HEIGHT / 2))) {
		(Equal, _) | (_, Equal) => None,
		(Less, Less) => Some(0),
		(Greater, Less) => Some(1),
		(Less, Greater) => Some(2),
		(Greater, Greater) => Some(3),
	}
}
