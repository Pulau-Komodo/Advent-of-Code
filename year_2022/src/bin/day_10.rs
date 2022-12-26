fn main() {
	shared::print_answers(10, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> Box<dyn std::fmt::Display> {
	let score: i32 = input
		.lines()
		.map(Instruction::from_str)
		.crt_processor()
		.enumerate()
		.skip(19)
		.step_by(40)
		.take(6)
		.map(|(i, value)| value * (i + 1) as i32)
		.sum();
	Box::new(score)
}

fn get_answer_2(input: &str) -> Box<dyn std::fmt::Display> {
	let output: String = input
		.lines()
		.map(Instruction::from_str)
		.crt_processor()
		.enumerate()
		.take(240)
		.flat_map(|(i, value)| {
			let char = if value.abs_diff(i as i32 % 40) <= 1 {
				'█'
			} else {
				' '
			};
			(i % 40 == 0).then_some('\n').into_iter().chain([char])
		})
		.collect();
	Box::new(output)
}

#[derive(Default)]
enum Instruction {
	#[default]
	Noop,
	Add(i32),
}

impl Instruction {
	fn from_str(str: &str) -> Self {
		if str == "noop" {
			Self::Noop
		} else {
			let (_, n) = str.split_once(' ').unwrap();
			Self::Add(n.parse().unwrap())
		}
	}
}

struct CrtProcessor<I> {
	value: i32,
	processing_for: u8,
	adding: i32,
	instructions: I,
}

impl<I> CrtProcessor<I> {
	fn new(instructions: I) -> Self {
		CrtProcessor {
			value: 1,
			processing_for: 0,
			adding: 0,
			instructions,
		}
	}
}

impl<I> Iterator for CrtProcessor<I>
where
	I: Iterator<Item = Instruction>,
{
	type Item = i32;

	fn next(&mut self) -> Option<Self::Item> {
		if self.processing_for > 0 {
			self.processing_for -= 1;
		} else {
			self.value += self.adding;
			let instruction = self.instructions.next().unwrap_or_default();
			self.adding = match instruction {
				Instruction::Noop => 0,
				Instruction::Add(n) => {
					self.processing_for = 1;
					n
				}
			}
		}
		Some(self.value)
	}
}

trait CrtInstructions {
	fn crt_processor(self) -> CrtProcessor<Self>
	where
		Self: Sized,
	{
		CrtProcessor::new(self)
	}
}

impl<T> CrtInstructions for T where T: Iterator<Item = Instruction> {}
