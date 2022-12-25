use std::collections::HashSet;

use shared::Vec3;

fn main() {
	shared::print_answers(18, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> usize {
	let voxels: HashSet<_> = input
		.lines()
		.map(Vec3::<i8>::from_comma_separated)
		.collect();
	voxels
		.iter()
		.flat_map(|voxel| voxel.orthogonal_neighbours())
		.filter(|voxel| !voxels.contains(voxel))
		.count()
}

fn get_answer_2(input: &str) -> usize {
	let voxels: HashSet<_> = input
		.lines()
		.map(Vec3::<i8>::from_comma_separated)
		.collect();
	let start = *voxels.iter().min_by_key(|voxel| voxel.x).unwrap();
	let start = Surface {
		voxel: start,
		direction: Direction::NegX,
	};
	let mut frontier = Vec::new();
	frontier.push(start);
	let mut new_frontier = HashSet::new();
	let mut visited = HashSet::new();
	visited.insert(start);
	loop {
		for surface in frontier.drain(..) {
			for (neighbour, folded_in) in surface
				.neighbours()
				.iter()
				.zip(surface.direction.folded_in())
			{
				let surface = if voxels.contains(&(*neighbour + surface.direction.up())) {
					// ▙
					Surface {
						voxel: *neighbour + surface.direction.up(),
						direction: folded_in.invert(),
					}
				} else if voxels.contains(neighbour) {
					// ▄
					Surface {
						voxel: *neighbour,
						direction: surface.direction,
					}
				} else {
					// ▗
					Surface {
						voxel: surface.voxel,
						direction: folded_in,
					}
				};
				if visited.insert(surface) {
					new_frontier.insert(surface);
				}
			}
		}
		if new_frontier.is_empty() {
			break;
		}
		frontier.extend(&new_frontier);
		new_frontier.clear();
	}
	visited.len()
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
	NegX,
	PosX,
	NegY,
	PosY,
	NegZ,
	PosZ,
}

impl Direction {
	fn offsets(self) -> [Vec3<i8>; 4] {
		use Direction::*;
		[(-1, 0), (1, 0), (0, -1), (0, 1)].map(|(a, b)| match self {
			NegX | PosX => Vec3 { x: 0, y: a, z: b },
			NegY | PosY => Vec3 { x: a, y: 0, z: b },
			NegZ | PosZ => Vec3 { x: a, y: b, z: 0 },
		})
	}
	fn folded_in(self) -> [Self; 4] {
		use Direction::*;
		match self {
			NegX | PosX => [NegY, PosY, NegZ, PosZ],
			NegY | PosY => [NegX, PosX, NegZ, PosZ],
			NegZ | PosZ => [NegX, PosX, NegY, PosY],
		}
	}
	fn invert(self) -> Self {
		match self {
			Self::NegX => Self::PosX,
			Self::PosX => Self::NegX,
			Self::NegY => Self::PosY,
			Self::PosY => Self::NegY,
			Self::NegZ => Self::PosZ,
			Self::PosZ => Self::NegZ,
		}
	}
	fn up(self) -> Vec3<i8> {
		match self {
			Self::NegX => Vec3 { x: -1, y: 0, z: 0 },
			Self::PosX => Vec3 { x: 1, y: 0, z: 0 },
			Self::NegY => Vec3 { x: 0, y: -1, z: 0 },
			Self::PosY => Vec3 { x: 0, y: 1, z: 0 },
			Self::NegZ => Vec3 { x: 0, y: 0, z: -1 },
			Self::PosZ => Vec3 { x: 0, y: 0, z: 1 },
		}
	}
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Surface {
	voxel: Vec3<i8>,
	direction: Direction,
}

impl Surface {
	fn neighbours(self) -> [Vec3<i8>; 4] {
		self.direction.offsets().map(|offset| self.voxel + offset)
	}
}
