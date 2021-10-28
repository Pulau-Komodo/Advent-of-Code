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

pub fn print_answers(day: u8, functions: &[fn(&str) -> String]) {
	let input = read_file(day);
	let mut now = std::time::Instant::now();
	if functions.len() == 1 {
		println!(
			"{} ({} μs)",
			functions[0](&input),
			now.elapsed().as_micros()
		);
		return;
	}
	for (index, function) in functions.iter().enumerate() {
		println!(
			"{}: {} ({} μs)",
			index + 1,
			function(&input),
			now.elapsed().as_micros()
		);
		now = std::time::Instant::now();
	}
}
