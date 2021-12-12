fn main() {
	shared::print_answers(12, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	let links = parse_input(input);
	find_routes(links, false)
}

fn get_answer_2(input: &str) -> u32 {
	let links = parse_input(input);
	find_routes(links, true)
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Cave {
	Start,
	End,
	Small(u8),
	Large(u8),
}

impl<'a, 'b> Cave {
	fn from_str_with_index_maps(
		str: &'a str,
		small_caves: &'b mut std::collections::HashMap<&'a str, u8>,
		large_caves: &'b mut std::collections::HashMap<&'a str, u8>,
	) -> Self {
		match str {
			"start" => Cave::Start,
			"end" => Cave::End,
			x if x.chars().next().unwrap().is_ascii_lowercase() => {
				let len = small_caves.len() as u8;
				let index = *small_caves.entry(x).or_insert(len);
				Cave::Small(index)
			}
			x => {
				let len = large_caves.len() as u8;
				let index = *large_caves.entry(x).or_insert(len);
				Cave::Large(index)
			}
		}
	}
}

fn parse_input(input: &str) -> Vec<(Cave, Cave)> {
	let capacity = input.lines().count();
	let mut small_caves = std::collections::HashMap::with_capacity(capacity / 2);
	let mut large_caves = std::collections::HashMap::with_capacity(capacity / 2);
	input
		.lines()
		.map(|line| {
			let (first, second) = line.split_once("-").unwrap();
			let first_cave =
				Cave::from_str_with_index_maps(first, &mut small_caves, &mut large_caves);
			let second_cave =
				Cave::from_str_with_index_maps(second, &mut small_caves, &mut large_caves);
			(first_cave, second_cave)
		})
		.collect()
}

#[derive(Clone)]
struct Route {
	last: Cave,
	small_caves: Vec<Cave>,
	revisited_small_cave: bool,
}

impl Route {
	fn new() -> Self {
		Self {
			last: Cave::Start,
			small_caves: Vec::new(),
			revisited_small_cave: false,
		}
	}
}

fn find_routes(links: Vec<(Cave, Cave)>, revisit_once: bool) -> u32 {
	let mut unfinished_routes = vec![Route::new()];
	let mut finished_routes = 0;
	loop {
		let mut new_unfinished_routes = Vec::new();
		for route in unfinished_routes {
			for next in links.iter().filter_map(|&(first, second)| {
				if route.last == first {
					Some(second)
				} else if route.last == second {
					Some(first)
				} else {
					None
				}
			}) {
				match next {
					Cave::Start => (),
					Cave::End => finished_routes += 1,
					Cave::Large(_) => {
						let mut route = route.clone();
						route.last = next;
						new_unfinished_routes.push(route);
					}
					Cave::Small(_) => {
						let visited = route.small_caves.contains(&next);
						if !visited || revisit_once && !route.revisited_small_cave {
							let mut route = route.clone();
							route.last = next;
							route.small_caves.push(next);
							if visited {
								route.revisited_small_cave = true;
							}
							new_unfinished_routes.push(route);
						}
					}
				}
			}
		}
		if new_unfinished_routes.is_empty() {
			return finished_routes;
		}
		unfinished_routes = new_unfinished_routes;
	}
}
