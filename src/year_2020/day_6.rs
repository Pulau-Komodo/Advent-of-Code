const QUESTIONS: [char; 26] = [
	'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
	's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];

pub fn part_a(input: String) -> String {
	let sum = input.split("\r\n\r\n").map(count_any_yes).sum::<u32>();
	format!("{}", sum)
}

fn count_any_yes(group: &str) -> u32 {
	QUESTIONS.iter().filter(|&&char| group.contains(char)).count() as u32
}

pub fn part_b(input: String) -> String {
	let sum = input.split("\r\n\r\n").map(count_all_yes).sum::<u32>();
	format!("{}", sum)
}

fn count_all_yes(group: &str) -> u32 {
	QUESTIONS.iter().filter(|&&char| {
		group.lines().all(|line| line.contains(char))
	}).count() as u32
}