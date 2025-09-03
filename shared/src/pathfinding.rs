pub struct BreadthFirstPathfinder<P, StepGen, Steps, Test>
where
	StepGen: FnMut(P) -> Steps,
	Steps: Iterator<Item = P>,
	Test: FnMut(&P) -> bool,
{
	frontier: Vec<P>,
	new_frontier: Vec<P>,
	step_gen: StepGen,
	test: Test,
}

impl<P, StepGen, Steps, Test> BreadthFirstPathfinder<P, StepGen, Steps, Test>
where
	StepGen: FnMut(P) -> Steps,
	Steps: Iterator<Item = P>,
	Test: FnMut(&P) -> bool,
{
	pub fn new(frontier: Vec<P>, step_gen: StepGen, test: Test) -> Self {
		let new_frontier = Vec::new();
		Self { 
			frontier, new_frontier, step_gen, test,
		}
	}
	pub fn progress(&mut self) -> bool {
		for position in self.frontier.drain(..) {
			for neighbour in (self.step_gen)(position) {
				if (self.test)(&neighbour) {
					self.new_frontier.push(neighbour);
				}
			}
		}
		std::mem::swap(&mut self.frontier, &mut self.new_frontier);
		!self.frontier.is_empty()
	}
}
