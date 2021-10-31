use std::iter::FromIterator;

fn main() {
	shared::print_answers(9, &[get_answers]);
}

fn get_answers(input: &str) -> String {
	let mut data = LocationData::from_str(input);
	let distances = data.all_distances();
	let (min, max) = distances
		.iter()
		.fold((u32::MAX, u32::MIN), |(min, max), &distance| {
			(min.min(distance), max.max(distance))
		});
	format!("1: {}, 2: {}", min, max)
}

struct LocationData<'l> {
	route: Vec<&'l str>,
	distances: std::collections::HashMap<(&'l str, &'l str), u32>,
}

impl<'l> LocationData<'l> {
	fn from_str(str: &'l str) -> Self {
		let mut set = std::collections::HashSet::new();
		let map = str
			.lines()
			.map(|line| {
				let (locations, distance) = line.split_once(" = ").unwrap();
				let locations = locations.split_once(" to ").unwrap();
				set.insert(locations.0);
				set.insert(locations.1);
				(locations, distance.parse().unwrap())
			})
			.collect();
		Self {
			route: Vec::from_iter(set),
			distances: map,
		}
	}
	fn get_distance(&self, first: &str, second: &str) -> u32 {
		*self
			.distances
			.get(&(first, second))
			.or_else(|| self.distances.get(&(second, first)))
			.unwrap()
	}
	fn route_distance(&self) -> u32 {
		self.route
			.windows(2)
			.map(|locations| self.get_distance(locations[0], locations[1]))
			.sum()
	}
	fn all_distances(&mut self) -> Vec<u32> {
		let mut distances = Vec::with_capacity(self.route.len());
		let mut c = vec![0; self.route.len()];
		let mut i = 0;
		distances.push(self.route_distance());
		while i < self.route.len() {
			if c[i] < i {
				if i % 2 == 0 {
					self.route.swap(0, i);
				} else {
					self.route.swap(c[i], i);
				}
				distances.push(self.route_distance());
				c[i] += 1;
				i = 0;
			} else {
				c[i] = 0;
				i += 1;
			}
		}
		distances
	}
}
