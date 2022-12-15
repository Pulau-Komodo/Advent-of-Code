use shared::{Point, RangeSet};

fn main() {
	shared::print_answers(15, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u64 {
	let sensor_count = input.lines().count();
	let mut beacons = Vec::with_capacity(sensor_count);
	let mut range_set = RangeSet::with_capacity(sensor_count);
	for sensor in input.lines().map(Sensor::from_str) {
		let distance_to_row = (sensor.position.y - ROW_TO_CHECK).abs();
		let manhattan_distance_on_row = sensor.manhattan_distance - distance_to_row;
		range_set.insert(
			-manhattan_distance_on_row + sensor.position.x
				..manhattan_distance_on_row + sensor.position.x + 1,
		);
		if sensor.closest_beacon.y == ROW_TO_CHECK && !beacons.contains(&sensor.closest_beacon.y) {
			beacons.push(sensor.closest_beacon.x);
		}
	}
	(range_set.len_sum() - beacons.len() as i32) as u64
}

fn get_answer_2(input: &str) -> u64 {
	let sensors: Vec<_> = input.lines().map(Sensor::from_str).collect();
	let mut point = Point::default();
	let mut range_set = RangeSet::with_capacity(sensors.len());
	for y in 0..=MAX_COORDINATES.y {
		for sensor in &sensors {
			let distance_to_row = (sensor.position.y - y).abs();
			let manhattan_distance_on_row = sensor.manhattan_distance - distance_to_row;
			range_set.insert(
				-manhattan_distance_on_row + sensor.position.x
					..manhattan_distance_on_row + sensor.position.x + 1,
			);
		}
		if let Some(gap) = range_set.gaps().next() {
			point = Point { x: gap.start, y };
			break;
		} else if range_set.start().unwrap() > 0 {
			point = Point { x: 0, y };
			break;
		} else if range_set.end().unwrap() < MAX_COORDINATES.x {
			point = Point {
				x: MAX_COORDINATES.x,
				y,
			};
			break;
		}
		range_set.clear();
	}
	point.x as u64 * MAX_COORDINATES.x as u64 + point.y as u64
}

const ROW_TO_CHECK: i32 = 2_000_000;
const MAX_COORDINATES: Point<i32> = Point {
	x: 4_000_000,
	y: 4_000_000,
};

struct Sensor {
	position: Point<i32>,
	closest_beacon: Point<i32>,
	manhattan_distance: i32,
}

impl Sensor {
	fn from_str(str: &str) -> Self {
		let (position, rest) = str[12..].split_once(':').unwrap();
		let (x, y) = position.split_once(',').unwrap();
		let position = Point {
			x: x.parse().unwrap(),
			y: y[3..].parse().unwrap(),
		};
		let (x, y) = rest[24..].split_once(',').unwrap();
		let closest_beacon = Point {
			x: x.parse().unwrap(),
			y: y[3..].parse().unwrap(),
		};
		let manhattan_distance = position.abs_diff(closest_beacon).component_sum();
		Self {
			position,
			closest_beacon,
			manhattan_distance,
		}
	}
}
