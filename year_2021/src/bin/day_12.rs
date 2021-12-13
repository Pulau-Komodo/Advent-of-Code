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

#[derive(Clone, Copy, Debug)]
enum Cave {
	Start,
	End,
	Small(usize),
	Large(usize),
}

impl<'a, 'b> Cave {
	fn from_str_with_index_map(
		str: &'a str,
		caves: &'b mut std::collections::HashMap<&'a str, usize>,
	) -> Self {
		match str {
			"start" => Cave::Start,
			"end" => Cave::End,
			x if x.chars().next().unwrap().is_ascii_lowercase() => {
				let len = caves.len();
				let index = *caves.entry(x).or_insert(len);
				Cave::Small(index)
			}
			x => {
				let len = caves.len();
				let index = *caves.entry(x).or_insert(len);
				Cave::Large(index)
			}
		}
	}
}

fn parse_input(input: &str) -> Vec<Vec<Cave>> {
	let capacity = input.lines().count();
	let mut caves = std::collections::HashMap::with_capacity(capacity);
	caves.insert("start", 0);
	let mut links: Vec<Vec<Cave>> = Vec::with_capacity(capacity * 2);
	links.push(Vec::new());
	for line in input.lines() {
		let (first, second) = line.split_once("-").unwrap();
		let first_cave = Cave::from_str_with_index_map(first, &mut caves);
		let second_cave = Cave::from_str_with_index_map(second, &mut caves);
		for (source, destination) in std::iter::once((first_cave, second_cave))
			.chain(std::iter::once((second_cave, first_cave)))
		{
			match (source, destination) {
				(Cave::End, _) => (),
				(_, Cave::Start) => (),
				(Cave::Start, dest) => links[0].push(dest),
				(Cave::Small(x) | Cave::Large(x), dest) => {
					while links.len() < x + 1 {
						links.push(Vec::new())
					}
					links[x].push(dest)
				}
			}
		}
	}
	links
}

struct Route {
	last: usize,
	small_caves: Vec<usize>,
	revisited_small_cave: bool,
}

impl Route {
	fn new() -> Self {
		Self {
			last: 0,
			small_caves: Vec::new(),
			revisited_small_cave: false,
		}
	}
	fn with_new_large_cave(other: &Self, cave: usize) -> Self {
		Self {
			last: cave,
			small_caves: other.small_caves.clone(),
			revisited_small_cave: other.revisited_small_cave,
		}
	}
	fn with_new_small_cave(other: &Self, cave: usize, revisited: bool) -> Self {
		let mut small_caves = Vec::with_capacity(other.small_caves.len() + !revisited as usize);
		small_caves.extend_from_slice(&other.small_caves);
		if !revisited {
			small_caves.push(cave);
		}
		Self {
			last: cave,
			small_caves,
			revisited_small_cave: other.revisited_small_cave || revisited,
		}
	}
}

fn find_routes(links: Vec<Vec<Cave>>, revisit_once: bool) -> u32 {
	let mut finished_routes = 0;
	let mut unfinished_routes = vec![Route::new()];
	let mut new_unfinished_routes = Vec::new();
	loop {
		for route in unfinished_routes {
			for &next in &links[route.last] {
				match next {
					Cave::End => finished_routes += 1,
					Cave::Large(x) => {
						let route = Route::with_new_large_cave(&route, x);
						new_unfinished_routes.push(route);
					}
					Cave::Small(x) => {
						let visited = route.small_caves.contains(&x);
						if !visited || revisit_once && !route.revisited_small_cave {
							let route = Route::with_new_small_cave(&route, x, visited);
							new_unfinished_routes.push(route);
						}
					}
					Cave::Start => unreachable!(),
				}
			}
		}
		if new_unfinished_routes.is_empty() {
			return finished_routes;
		}
		unfinished_routes = new_unfinished_routes;
		new_unfinished_routes = Vec::new();
	}
}
