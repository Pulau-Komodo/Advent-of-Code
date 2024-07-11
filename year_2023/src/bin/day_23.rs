use shared::{Grid, Offset, Point};

fn main() {
	shared::print_answers(23, &[get_answer_1, get_answer_2]);
}

// Hardcoded: all intersections are encapsulated by slopes, and slopes only appear at intersections. All slopes are south or east.
fn get_answer_1(input: &str) -> usize {
	solve(input, true)
}

fn get_answer_2(input: &str) -> usize {
	solve(input, false)
}

fn solve(input: &str, is_icy: bool) -> usize {
	let grid = Grid::with_margin(
		input.lines().map(|line| line.chars().map(Tile::from_char)),
		Tile::Forest,
	);
	let start = grid
		.iter_with_points::<usize>()
		.find_map(|(point, tile)| (*tile == Tile::Path).then_some(point))
		.unwrap();
	let end = grid
		.iter_with_points::<usize>()
		.rev()
		.find_map(|(point, tile)| (*tile == Tile::Path).then_some(point))
		.unwrap();
	let processed_grid = find_intersections(&grid);
	let mut links = compile_grid_into_network(&processed_grid, start, end);

	if !is_icy {
		let reversed_links = links
			.iter()
			.filter(|link| link.start != start && link.end != end)
			.map(Link::reverse)
			.collect::<Vec<_>>();
		links.extend(reversed_links);
	}

	let starting_link = *links.iter().find(|link| link.start == start).unwrap();
	let mut frontier = vec![vec![starting_link]];
	let mut found_paths = Vec::new();
	loop {
		let mut new_frontier = Vec::new();
		for history in frontier.drain(..) {
			let node = history.last().unwrap().end;
			for link in links.iter().filter(|link| link.start == node) {
				if history.iter().any(|past_link| link.end == past_link.start) {
					continue;
				}
				let mut history = history.clone();
				history.push(*link);
				if link.end == end {
					found_paths.push(history)
				} else {
					new_frontier.push(history);
				}
			}
		}
		if new_frontier.is_empty() {
			break;
		}
		std::mem::swap(&mut frontier, &mut new_frontier);
	}

	found_paths
		.into_iter()
		.map(|path| path.into_iter().map(|link| link.length).sum::<usize>())
		.max()
		.unwrap()
}

fn find_intersections(grid: &Grid<Tile>) -> Grid<ProcessedTile> {
	let mut processed_grid = Grid::empty(grid.width(), grid.height(), ProcessedTile::Forest);
	for ((point, tile), processed) in grid
		.iter_with_points::<usize>()
		.zip(processed_grid.iter_mut())
	{
		match tile {
			Tile::Path => {
				if point
					.orthogonal_neighbours()
					.into_iter()
					.filter(|neighbour| {
						matches!(
							grid.get_point(*neighbour),
							Tile::SlopeEast | Tile::SlopeSouth
						)
					})
					.count() >= 3
				{
					*processed = ProcessedTile::Intersection;
				} else {
					*processed = ProcessedTile::Path;
				}
			}
			Tile::SlopeEast | Tile::SlopeSouth => {
				*processed = ProcessedTile::Path;
			}
			_ => (),
		}
	}
	processed_grid
}

fn compile_grid_into_network(
	grid: &Grid<ProcessedTile>,
	start: Point<usize>,
	end: Point<usize>,
) -> Vec<Link> {
	let mut links = Vec::new();
	let mut visited = Grid::empty(grid.width(), grid.height(), false);
	let mut found_nodes = vec![start];
	let mut nodes_to_explore = vec![start];
	while let Some(node) = nodes_to_explore.pop() {
		for mut position in [node + Offset::new(1, 0), node + Offset::new(0, 1)] {
			// Start at both valid offsets from the start.
			if grid.get_point(position) == ProcessedTile::Forest {
				// Skip the ones that are not walkable.
				continue;
			}
			*visited.get_point_mut(node) = true;
			*visited.get_point_mut(position) = true;
			let mut steps = 1;
			'outer: loop {
				let mut has_moved = false;
				for neighbour in position.orthogonal_neighbours() {
					match grid.get_point(neighbour) {
						ProcessedTile::Forest => (),
						ProcessedTile::Path => {
							if neighbour == end {
								links.push(Link {
									start: node,
									end,
									length: steps + 1,
								}); // Found end, make link but don't search further
								break 'outer;
							} else if !visited.get_point(neighbour) {
								*visited.get_point_mut(neighbour) = true;
								position = neighbour;
								has_moved = true;
								steps += 1;
								break;
							}
						}
						ProcessedTile::Intersection => {
							if neighbour.x > position.x || neighbour.y > position.y {
								// Found crossroads from valid direction, make link and optionally search from it
								links.push(Link {
									start: node,
									end: neighbour,
									length: steps + 1,
								});
								if !found_nodes.contains(&neighbour) {
									found_nodes.push(neighbour);
									nodes_to_explore.push(neighbour);
								}
							} else if neighbour != node {
								// Only break if this isn't where we just came from
								break 'outer;
							}
						}
					}
				}
				if !has_moved {
					break;
				}
			}
		}
	}
	links
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
	Forest,
	Path,
	SlopeEast,
	SlopeSouth,
}

impl Tile {
	fn from_char(char: char) -> Self {
		match char {
			'#' => Self::Forest,
			'.' => Self::Path,
			'>' => Self::SlopeEast,
			'v' => Self::SlopeSouth,
			_ => panic!("Unexpected character"),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ProcessedTile {
	Forest,
	Path,
	Intersection,
}

#[derive(Debug, Clone, Copy)]
struct Link {
	start: Point<usize>,
	end: Point<usize>,
	length: usize,
}

impl Link {
	fn reverse(&self) -> Self {
		Self {
			start: self.end,
			end: self.start,
			length: self.length,
		}
	}
}
