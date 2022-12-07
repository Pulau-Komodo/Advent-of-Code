use std::collections::HashMap;

fn main() {
	shared::print_answers(7, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	let mut state = State::default();
	for output in input.lines().skip(1).map(TerminalOutput::from_str) {
		state.handle_terminal_output(output);
	}
	let mut sum = 0;
	state.for_each_directory(|directory| {
		if directory.size <= SMALL_DIRECTORY_THRESHOLD {
			sum += directory.size
		}
	});
	sum
}

fn get_answer_2(input: &str) -> u32 {
	let mut state = State::default();
	for output in input.lines().skip(1).map(TerminalOutput::from_str) {
		state.handle_terminal_output(output);
	}
	let threshold = state.root.size - (DISK_SPACE - SPACE_NEEDED);
	let mut smallest = u32::MAX;
	state.for_each_directory(|directory| {
		if directory.size >= threshold {
			smallest = smallest.min(directory.size)
		}
	});
	smallest
}

const SMALL_DIRECTORY_THRESHOLD: u32 = 100_000;
const DISK_SPACE: u32 = 70_000_000;
const SPACE_NEEDED: u32 = 30_000_000;

enum TerminalOutput<'l> {
	ChangeDirectory(&'l str),
	ParentDirectory,
	List,
	Directory(&'l str),
	File(u32),
}

impl<'l> TerminalOutput<'l> {
	fn from_str(str: &'l str) -> Self {
		let mut elements = str.split(' ');
		match (
			elements.next().unwrap(),
			elements.next().unwrap(),
			elements.next(),
		) {
			("$", "cd", Some("..")) => Self::ParentDirectory,
			("$", "cd", Some(dir)) => Self::ChangeDirectory(dir),
			("$", "ls", _) => Self::List,
			("dir", name, _) => Self::Directory(name),
			(n, _name, _) => Self::File(n.parse().unwrap()),
		}
	}
}

#[derive(Default, Debug)]
struct Directory<'l> {
	directories: HashMap<&'l str, Directory<'l>>,
	size: u32,
}

#[derive(Default, Debug)]
struct State<'l> {
	root: Directory<'l>,
	current_path: Vec<&'l str>,
}

impl<'l> State<'l> {
	fn handle_terminal_output(&mut self, output: TerminalOutput<'l>) {
		match output {
			TerminalOutput::ChangeDirectory(directory) => self.change_directory(directory),
			TerminalOutput::ParentDirectory => self.change_to_parent_directory(),
			TerminalOutput::List => (),
			TerminalOutput::Directory(name) => self.add_directory(name),
			TerminalOutput::File(size) => self.add_file(size),
		}
	}
	fn add_directory(&mut self, name: &'l str) {
		let mut position = &mut self.root;
		for step in &self.current_path {
			let new_position = position.directories.get_mut(*step).unwrap();
			position = new_position;
		}
		position.directories.insert(name, Directory::default());
	}
	fn add_file(&mut self, size: u32) {
		let mut position = &mut self.root;
		position.size += size;
		for step in &self.current_path {
			let new_position = position.directories.get_mut(*step).unwrap();
			position = new_position;
			position.size += size;
		}
	}
	fn change_directory(&mut self, directory: &'l str) {
		self.current_path.push(directory);
	}
	fn change_to_parent_directory(&mut self) {
		self.current_path.pop();
	}
	fn for_each_directory<F>(&self, mut f: F)
	where
		F: FnMut(&Directory),
	{
		let mut frontier: Vec<_> = self.root.directories.values().collect();
		loop {
			let mut new_frontier = Vec::new();
			for directory in frontier.drain(..) {
				f(directory);
				new_frontier.extend(directory.directories.values());
			}
			if new_frontier.is_empty() {
				break;
			}
			frontier.append(&mut new_frontier);
		}
	}
}
