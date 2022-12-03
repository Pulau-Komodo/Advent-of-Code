mod md5;
mod top;

pub use md5::md5;
pub use top::IteratorTop;

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

/// Prints a line with the passed value, and returns true. For debugging big boolean chains.
pub fn println<T: std::fmt::Display>(output: T) -> bool {
	println!("{}", output);
	true
}

pub fn print_answers<T: std::fmt::Display>(day: u8, functions: &[fn(&str) -> T]) {
	let repeat_count: u32 = std::env::args()
		.nth(1)
		.and_then(|arg| arg.parse().ok())
		.unwrap_or(1);
	let input = read_file(day);
	let mut now = std::time::Instant::now();
	if functions.len() == 1 {
		for _ in 1..repeat_count {
			functions[0](&input);
		}
		println!(
			"{} ({} μs)",
			functions[0](&input),
			now.elapsed().as_micros() / repeat_count as u128
		);
	} else {
		for (index, function) in functions.iter().enumerate() {
			for _ in 1..repeat_count {
				function(&input);
			}
			println!(
				"{}: {} ({} μs)",
				index + 1,
				function(&input),
				now.elapsed().as_micros() / repeat_count as u128
			);
			now = std::time::Instant::now();
		}
	}
	if repeat_count > 1 {
		println!("(times averaged over {repeat_count} runs)");
	}
}
