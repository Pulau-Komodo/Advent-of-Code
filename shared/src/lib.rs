mod grid;
mod internal;
mod io;
mod md5;
mod parsing;
mod point;
mod range_set;
mod top;
mod vectors;

pub use grid::Grid;
pub use io::{print_answers, read_file, read_file_special};
pub use md5::md5;
pub use parsing::{bytes_to_integer, split_number};
pub use point::{FlatPoint, Offset, Point};
pub use range_set::RangeSet;
pub use top::IteratorTop;
pub use vectors::{Vec2, Vec3};

/// Prints a line with the passed value, and returns true. For debugging big boolean chains.
pub fn println<T: std::fmt::Display>(output: T) -> bool {
	println!("{}", output);
	true
}
