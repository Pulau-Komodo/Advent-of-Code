use std::fs;

use shared::{Offset, Point};

fn main() {
	shared::print_answers(10, &[get_answers]);
}

fn get_answers(input: &str) -> u32 {
	let mut stars: Vec<_> = input.lines().map(star_from_line).collect();
	let mut seconds = 0;
	let mut been_within_threshold = false;
	loop {
		let (min, max) = stars.iter().map(|(pos, _vel)| pos).fold(
			(
				Point::new(i32::MAX, i32::MAX),
				Point::new(i32::MIN, i32::MIN),
			),
			|(acc_min, acc_max), point| {
				(acc_min.component_min(*point), acc_max.component_max(*point))
			},
		);
		stars.sort_by_key(|(pos, _)| (pos.y, pos.x));
		const RENDER_THRESHOLD: i32 = 100;
		let size = max.abs_diff(min);
		if size.x <= RENDER_THRESHOLD && size.y <= RENDER_THRESHOLD {
			been_within_threshold = true;
			println!("{seconds}");
			let mut pos = min;
			let mut image = String::new();
			if stars[0].0 == min {
				image.push('█');
			}
			for (point, _) in &stars {
				if *point == pos {
					continue;
				}
				if point.y > pos.y {
					// New line
					for _ in pos.y..point.y {
						image.push('\n');
					}
					for _ in min.x..point.x - 1 {
						image.push(' ');
					}
				} else {
					for _ in pos.x..point.x - 1 {
						image.push(' ');
					}
				}
				image.push('█');
				pos = *point;
			}
			fs::write("day_10_output.txt", image).unwrap();
			let mut ignore = String::new();
			std::io::stdin().read_line(&mut ignore).unwrap();
		} else if been_within_threshold {
			return 0;
		}
		for point in &mut stars {
			point.0 += point.1;
		}
		seconds += 1;
	}
}

fn star_from_line(line: &str) -> (Point<i32>, Offset<i32>) {
	let (position, velocity) = line.split_once("> ").unwrap();
	let mut nums = [&position[10..]]
		.into_iter()
		.chain([&velocity[10..velocity.len() - 1]])
		.flat_map(|nums| {
			let (x, y) = nums.split_once(',').unwrap();
			[x, y]
		})
		.map(|num| num.trim().parse().unwrap());
	let position = Point::new(nums.next().unwrap(), nums.next().unwrap());
	let velocity = Offset::new(nums.next().unwrap(), nums.next().unwrap());
	(position, velocity)
}
