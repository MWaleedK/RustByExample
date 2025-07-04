pub type GraphemeIdx = usize;
pub type LineIdx = usize;
pub type ByteIdx = usize;
pub type ColIdx = usize;
pub type RowIdx = usize;

mod position;
pub use position::Position;
mod size;
pub use size::Size;
mod location;
pub use location::Location;
pub const NAME: &str = env!("CARGO PKG NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
