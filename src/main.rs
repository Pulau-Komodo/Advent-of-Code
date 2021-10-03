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
		_ => unimplemented!(),
	};
	println!("{}", result);
}
