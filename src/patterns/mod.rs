//! Iterators over common [`Coord`](crate::coord::Coord) shapes and patterns.
//!
//! These patterns have no dependencies on actual cell data. Their intended use
//! is for ultimately passing into
//! [`Grid::selection_iter`](crate::grid::Grid::selection_iter) or
//! [`Grid::selection_iter_mut`](crate::grid::Grid::selection_iter_mut) to obtain
//! actual cell values.

mod cluster;
mod line;
mod neighborhood;
mod rect;

pub use cluster::{Cluster, ExternalBorderIter};
pub use line::{Line, LineIter};
pub use neighborhood::Neighborhood;
pub use rect::{BspTree, Orientation, Rect, RectIter};
