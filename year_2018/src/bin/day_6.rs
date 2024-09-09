use shared::{IntoCartesianProduct, Point};

fn main() {
	shared::print_answers(6, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	let points: Vec<Point<u32>> = input
		.lines()
		.map(|line| {
			let (x, y) = line.split_once(", ").unwrap();
			Point::new(x.parse().unwrap(), y.parse().unwrap())
		})
		.collect();
	let max = points.iter().fold(Point::new(0, 0), |acc, point| {
		Point::new(acc.x.max(point.x), acc.y.max(point.y))
	});
	let mut regions = vec![Some(0); points.len()];
	for (x, y) in (0..=max.x).cartesian_product(0..=max.y) {
		let test_point = Point::new(x, y);
		let (closest, _point) = points
			.iter()
			.enumerate()
			.min_by_key(|(_, point)| point.abs_diff(test_point).component_sum())
			.unwrap();
		if test_point.x == 0 || test_point.x == max.x || test_point.y == 0 || test_point.y == max.y
		{
			regions[closest] = None;
		} else if let Some(region_size) = regions[closest] {
			regions[closest] = Some(region_size + 1);
		}
	}
	regions.into_iter().flatten().max().unwrap()
}

fn get_answer_2(input: &str) -> u32 {
	let points: Vec<Point<i32>> = input
		.lines()
		.map(|line| {
			let (x, y) = line.split_once(", ").unwrap();
			Point::new(x.parse().unwrap(), y.parse().unwrap())
		})
		.collect();
	let (min, max) = points.iter().fold(
		(
			Point::new(i32::MAX, i32::MAX),
			Point::new(i32::MIN, i32::MIN),
		),
		|(min_acc, max_acc), point| (min_acc.component_min(*point), max_acc.component_max(*point)),
	);
	const DISTANCE_LIMIT: i32 = 10000 - 1;
	let start = Point::new(max.x - DISTANCE_LIMIT, max.y - DISTANCE_LIMIT);
	let end = Point::new(min.x + DISTANCE_LIMIT, min.y + DISTANCE_LIMIT);
	let mut region_size = 0;
	'outer: for (x, y) in (start.x..=end.x).cartesian_product(start.y..=end.y) {
		let test_point = Point::new(x, y);
		let mut sub_sum = 0;
		for point in &points {
			sub_sum += point.abs_diff(test_point).component_sum();
			if sub_sum > DISTANCE_LIMIT {
				continue 'outer;
			}
		}
		region_size += 1;
	}
	region_size
}
