use std::collections::HashMap;

fn main() {
	shared::print_answers(7, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> String {
	let programs: HashMap<_, _> = input.lines().map(Program::from_line).collect();
	programs
		.keys()
		.find(|program_name| {
			!programs
				.values()
				.any(|program| program.supporting.contains(program_name))
		})
		.unwrap()
		.to_string()
}

fn get_answer_2(input: &str) -> String {
	let programs: HashMap<_, _> = input.lines().map(Program::from_line).collect();
	let root = *programs
		.keys()
		.find(|program_name| {
			!programs
				.values()
				.any(|program| program.supporting.contains(program_name))
		})
		.unwrap();
	let mut programs_generations = HashMap::with_capacity(programs.len());
	let mut frontier = vec![root];
	let mut generation = 0;
	loop {
		let mut new_frontier = Vec::new();
		for frontier_program in frontier.drain(..) {
			let program = programs.get(frontier_program).unwrap();
			new_frontier.extend(program.supporting.iter().copied());
			programs_generations.insert(frontier_program, generation);
		}
		if new_frontier.is_empty() {
			break;
		}
		generation += 1;
		std::mem::swap(&mut frontier, &mut new_frontier);
	}

	let mut programs_cumulative_weight = programs.clone();

	for generation in (0..generation).rev() {
		for name in programs_generations
			.iter()
			.filter_map(|(name, gen)| (*gen == generation).then_some(*name))
		{
			let supporting = programs.get(name).unwrap().supporting.clone();
			let weight = supporting
				.iter()
				.map(|program| programs_cumulative_weight.get(program).unwrap().weight)
				.sum::<i32>();
			programs_cumulative_weight.get_mut(name).unwrap().weight += weight;
		}
	}
	let programs_cumulative_weight = programs_cumulative_weight;

	let mut difference = None;
	let mut frontier = vec![root];
	loop {
		let mut new_frontier = Vec::new();
		for frontier_program in frontier.drain(..) {
			let supporting = &programs.get(frontier_program).unwrap().supporting;
			let Some(outlier) = position_odd_one_out(
				supporting
					.iter()
					.map(|program| programs_cumulative_weight.get(program).unwrap().weight),
			) else {
				return format!(
					"{}",
					programs.get(frontier_program).unwrap().weight + difference.unwrap()
				);
			};
			// // Hardcoded: this just doesn't come up so I'm not going to bother handling it.
			// if supporting.len() == 2 {
			// 	for program_name in supporting {
			// 		let program = programs.get(program_name).unwrap();
			// 		new_frontier.extend(program.supporting.iter().copied());
			// 	}
			// }
			if difference.is_none() {
				// This could be outside the loop only if it branches into multiple right at the start, which it does. I considered moving it out, but having it here is theoretically the right move if it doesn't branch at the start, so I won't bother.
				difference = Some(
					programs_cumulative_weight
						.get(supporting[(outlier + 1) % supporting.len()])
						.unwrap()
						.weight - programs_cumulative_weight
						.get(supporting[outlier])
						.unwrap()
						.weight,
				);
			}
			new_frontier.push(supporting[outlier]);
		}
		if new_frontier.is_empty() {
			break;
		}
		std::mem::swap(&mut frontier, &mut new_frontier);
	}
	"Whoops".into()
}

#[derive(Debug, Clone)]
struct Program<'l> {
	weight: i32,
	supporting: Vec<&'l str>,
}

impl<'l> Program<'l> {
	fn from_line(line: &'l str) -> (&'l str, Self) {
		let (program, supporting) = line
			.split_once(" -> ")
			.map(|(program, supporting)| (program, Some(supporting)))
			.unwrap_or((line, None));
		let (name, weight) = program.split_once(' ').unwrap();
		let weight = weight[1..weight.len() - 1].parse().unwrap();
		let supporting = supporting
			.iter()
			.flat_map(|text| text.split(", "))
			.collect();
		(name, Program { weight, supporting })
	}
}

fn position_odd_one_out<T: PartialEq>(iter: impl IntoIterator<Item = T>) -> Option<usize> {
	let mut iter = iter.into_iter();
	let first = iter.next()?;
	let first_different = iter.position(|item| item != first)?;
	if first_different > 0 {
		Some(first_different + 1)
	} else if let Some(third) = iter.next() {
		if first == third {
			Some(1)
		} else {
			Some(0)
		}
	} else {
		panic!("There were only two items");
	}
}
