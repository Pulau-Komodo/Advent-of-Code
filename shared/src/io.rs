use std::{fmt::Display, hint::black_box};

pub fn read_file(day: u8) -> String {
	let path_adjustment = if cfg!(test) { "." } else { "" };
	std::fs::read_to_string(format!(".{}/input/{}.txt", path_adjustment, day))
		.expect("Could not read file")
}

pub fn read_file_special(day: u8, addition: &str) -> String {
	let path_adjustment = if cfg!(test) { "." } else { "" };
	std::fs::read_to_string(format!(
		".{}/input/{}_{}.txt",
		path_adjustment, day, addition
	))
	.expect("Could not read file")
}

pub fn print_answers<T>(day: u8, solvers: &'static [fn(&str) -> T])
where
	T: Display,
{
	if solvers.len() == 1 {
		print_labelled_answers(day, solvers.iter().map(|solver| ("", *solver)));
	} else {
		print_labelled_answers(
			day,
			solvers
				.iter()
				.enumerate()
				.map(|(index, solver)| (index + 1, *solver)),
		);
	}
}

pub fn print_labelled_answers<T, N, I>(day: u8, solvers: I)
where
	T: Display + 'static,
	N: Display,
	I: IntoIterator<Item = (N, fn(&str) -> T)>,
{
	let repeat_count: u32 = std::env::args()
		.nth(1)
		.and_then(|arg| arg.parse().ok())
		.unwrap_or(1);
	let input = read_file(day);
	let mut now = std::time::Instant::now();
	for (name, function) in solvers {
		for _ in 1..repeat_count {
			black_box(function(&input));
		}
		let result = function(&input);
		let elapsed = now.elapsed().as_micros() / repeat_count as u128;
		if name.to_string().is_empty() {
			println!("{result} ({elapsed} μs)");
		} else {
			println!("{name}: {result} ({elapsed} μs)");
		}
		now = std::time::Instant::now();
	}
	if repeat_count > 1 {
		println!("(times averaged over {repeat_count} runs)");
	}
}
