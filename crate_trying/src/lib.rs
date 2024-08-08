//	# 14.3
//	- use the 'pub use' can re-export the types in the library
// when crate is published, this version of crate will be undeleted and cannot be covered.
// - to publish a new version crate, you need to change the version number in Cargo.toml file, or it
// 		cannot be published.
// - used cargo yank --vers 0.1.0 to remove the version 0.1.0 from the crates.io,then other new
// 		projects cannot use this version. But the version is still in the crates.io, and if project have
// 		used it before you yank this crate,it will be work.
// - used cargo yank --vers 0.1.0 --undo to undo the yank operation.

//! # Art
//!
//! A library for modeling artistic concepts.

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;
/*	or you can also use following:
pub use kinds::PrimaryColor;
pub use kinds::SecondaryColor;
pub use utils::mix;
*/

pub mod kinds {
	/// the primary colors according to the RYB color model
	pub enum PrimaryColor {
		Red,
		Yellow,
		Blue,
	}

	/// the secondary colors according to the RYB color model
	pub enum SecondaryColor {
		Orange,
		Green,
		Purple,
	}
}

pub mod utils {
	use crate ::kinds::*;

	/// Combines two primary colors in equal amounts to create a secondary color.
	/// a secondary color.
	pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
		// --snip--
		SecondaryColor::Green
	}
}