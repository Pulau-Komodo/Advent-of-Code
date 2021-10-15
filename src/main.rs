mod year_2020;

fn main() {
	let mut args = std::env::args().skip(1);
	let year = args.next().expect("Specify year");
	let day = args.next().expect("Specify day");
	let part = args.next().unwrap_or_else(|| String::from("a"));
	let input = std::fs::read_to_string(format!("./input/{}/{}.txt", year, day))
		.expect("Could not read file");
	let result = match (year.as_str(), day.as_str(), part.as_str()) {
		("2020", "1", "a") => year_2020::day_1::part_a(input),
		("2020", "1", "b") => year_2020::day_1::part_b(input),
		("2020", "2", "a") => year_2020::day_2::part_a(input),
		("2020", "2", "b") => year_2020::day_2::part_b(input),
		("2020", "3", "a") => year_2020::day_3::part_a(input),
		("2020", "3", "b") => year_2020::day_3::part_b(input),
		("2020", "4", "a") => year_2020::day_4::part_a(input),
		("2020", "4", "b") => year_2020::day_4::part_b(input),
		("2020", "5", "a") => year_2020::day_5::part_a(input),
		("2020", "5", "b") => year_2020::day_5::part_b(input),
		("2020", "6", "a") => year_2020::day_6::part_a(input),
		("2020", "6", "b") => year_2020::day_6::part_b(input),
		("2020", "7", "a") => year_2020::day_7::part_a(input),
		("2020", "7", "b") => year_2020::day_7::part_b(input),
		("2020", "8", "a") => year_2020::day_8::part_a(input),
		("2020", "8", "b") => year_2020::day_8::part_b(input),
		("2020", "9", _) => year_2020::day_9::get_answers(input),
		("2020", "10", _) => year_2020::day_10::get_answers(input),
		("2020", "11", _) => year_2020::day_11::get_answers(input),
		("2020", "12", _) => year_2020::day_12::get_answers(input),
		("2020", "13", _) => year_2020::day_13::get_answers(input),
		("2020", "14", _) => year_2020::day_14::get_answers(input),
		("2020", "15", _) => year_2020::day_15::get_answers(input),
		("2020", "16", _) => year_2020::day_16::get_answers(input),
		_ => unimplemented!(),
	};
	println!("{}", result);
}
