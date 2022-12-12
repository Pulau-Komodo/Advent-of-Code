mod grid;
mod io;
mod md5;
mod point;
mod top;

pub use grid::Grid;
pub use io::{print_answers, read_file, read_file_special};
pub use md5::md5;
pub use point::{FlatPoint, Offset, Point};
pub use top::IteratorTop;

/// Prints a line with the passed value, and returns true. For debugging big boolean chains.
pub fn println<T: std::fmt::Display>(output: T) -> bool {
	println!("{}", output);
	true
}
