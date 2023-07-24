
pub mod split;
pub mod auto_complete;
pub mod square_grid;
pub mod detail;
pub mod float_right;
pub mod separator;

pub mod prelude {
	pub use super::split::{Split,SplitProps, SplitDirection};
	pub use super::auto_complete::SimpleWordComplete;
	pub use super::square_grid::{SquareGrid, SquareGridProps};
	pub use super::detail::{Detail, DetailProps};
	pub use super::float_right::{FloatRight, FloatRightProps};
	pub use super::separator::{HR,VR};
}