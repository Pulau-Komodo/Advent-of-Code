mod bitmask;
mod cartesian_product;
mod direction;
mod grid;
mod internal;
mod io;
mod math;
mod md5;
mod pair_iterator;
mod parsing;
mod point;
mod range;
mod range_set;
mod small_map;
mod top;
mod vectors;

pub use bitmask::{Bit, Bitmask};
pub use cartesian_product::{CartesianProduct, IntoCartesianProduct};
pub use direction::{Direction, Direction8};
pub use grid::Grid;
pub use io::{print_answers, print_labelled_answers, read_file, read_file_special};
pub use math::{count_digits, div_ceil, wrapping_add, wrapping_sub};
pub use md5::md5;
pub use pair_iterator::{IntoPairIterator, PairIterator};
pub use parsing::{bytes_to_integer, split_number, try_split_number};
pub use point::{FlatPoint, Offset, Point};
pub use range::Range;
pub use range_set::{RangeDoubleExclusive, RangeInclusiveSet, RangeSet};
pub use small_map::SmallMap;
pub use top::IteratorTop;
pub use vectors::{Vec2, Vec3};

/// Prints a line with the passed value, and returns true. For debugging big boolean chains.
pub fn println<T: std::fmt::Display>(output: T) -> bool {
	println!("{}", output);
	true
}
