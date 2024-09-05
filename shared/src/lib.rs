mod bitmask;
mod grid;
mod internal;
mod io;
mod math;
mod md5;
mod parsing;
mod point;
mod range_set;
mod top;
mod vectors;
mod small_map;

pub use bitmask::{Bit, Bitmask};
pub use grid::Grid;
pub use io::{print_answers, print_labelled_answers, read_file, read_file_special};
pub use math::{div_ceil, wrapping_add, wrapping_sub};
pub use md5::md5;
pub use parsing::{bytes_to_integer, split_number};
pub use point::{FlatPoint, Offset, Point};
pub use range_set::{RangeDoubleExclusive, RangeInclusiveSet, RangeSet};
pub use top::IteratorTop;
pub use vectors::{Vec2, Vec3};
pub use small_map::SmallMap;

/// Prints a line with the passed value, and returns true. For debugging big boolean chains.
pub fn println<T: std::fmt::Display>(output: T) -> bool {
	println!("{}", output);
	true
}
