pub mod colors;
pub mod render;
pub mod structures;

#[cfg(feature = "shim")]
pub mod shim;
#[cfg(feature = "shim")]
pub use shim::*;

#[cfg(not(feature = "shim"))]
pub use screeps::constants::StructureType;
