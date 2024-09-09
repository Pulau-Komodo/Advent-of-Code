use shared::SmallMap;

fn main() {
	shared::print_answers(7, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> String {
	let (mut dependants, dependencies) = parse_relationships(input);

	let mut queue = find_independents(&dependants, &dependencies);
	let mut output = String::new();
	while !queue.is_empty() {
		let step_index = queue
			.iter()
			.enumerate()
			.min_by_key(|(_, step)| **step)
			.unwrap()
			.0;
		let step = queue.remove(step_index);
		output.push(step as char);
		for dependant in dependants.remove(&step).into_iter().flatten() {
			if dependencies
				.get(&dependant)
				.unwrap()
				.iter()
				.all(|dependency| output.contains(*dependency as char))
			{
				queue.push(dependant);
			}
		}
	}
	output
}

fn get_answer_2(input: &str) -> String {
	let (mut dependants, dependencies) = parse_relationships(input);

	let mut queue = find_independents(&dependants, &dependencies);
	let mut worker_tasks = Vec::new();
	let mut output = String::new();
	const WORKER_COUNT: usize = 4;
	let mut time_passed = 0;
	while !queue.is_empty() || !worker_tasks.is_empty() {
		time_passed += 1;
		queue.sort_unstable_by(|a, b| b.cmp(a));
		while worker_tasks.len() < WORKER_COUNT && !queue.is_empty() {
			let step = queue.pop().unwrap();
			worker_tasks.push((step, step - b'A' + 61));
		}
		for (step, time) in &mut worker_tasks {
			*time -= 1;
			if *time == 0 {
				output.push(*step as char);
				for dependant in dependants.remove(&*step).into_iter().flatten() {
					if dependencies
						.get(&dependant)
						.unwrap()
						.iter()
						.all(|dependency| output.contains(*dependency as char))
					{
						queue.push(dependant);
						queue.sort_unstable_by(|a, b| b.cmp(a));
					}
				}
			}
		}
		worker_tasks.retain(|(_step, time)| *time > 0);
	}
	time_passed.to_string()
}

fn parse_relationships(input: &str) -> (SmallMap<u8, Vec<u8>>, SmallMap<u8, Vec<u8>>) {
	let mut dependants = SmallMap::new();
	let mut dependencies = SmallMap::new();
	for line in input.lines() {
		let dependency = line.as_bytes()[5];
		let dependant = line.as_bytes()[36];
		dependants
			.get_mut_or_insert(dependency, Vec::new())
			.push(dependant);
		dependencies
			.get_mut_or_insert(dependant, Vec::new())
			.push(dependency);
	}
	(dependants, dependencies)
}

fn find_independents(
	dependants: &SmallMap<u8, Vec<u8>>,
	dependencies: &SmallMap<u8, Vec<u8>>,
) -> Vec<u8> {
	dependants
		.keys()
		.filter(|dependency| !dependencies.contains_key(dependency))
		.copied()
		.collect()
}
