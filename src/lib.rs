/*
Sharkey.rs by Alyx Shang.
Licensed under the FSL v1.
*/

/// Declaring the "modules"
/// directory as a module.
pub mod modules;

/// Re-exporting the module
/// containing this crate's
/// CLI.
#[cfg(feature="cli")]
pub use modules::cli::*;

/// Re-exporting the
/// structures to 
/// "help" the main
/// structures.
pub use modules::extra::*;

/// Re-exporting the 
/// module to handle
/// errors.
pub use modules::error::*;

/// Re-exporting the 
/// module with needed
/// enums.
pub use modules::enums::*;

/// Re-exporting the module
/// that contains a bridging
/// type for handling requests.
pub use modules::bridge::*;

/// Re-exporting the module
/// containing actions the
/// user needs to be authenticated
/// for.
pub use modules::actions::*;

/// Re-exporting the 
/// module to handle
/// network requests.
pub use modules::network::*;

/// Re-exporting the module
/// that contains serializable
/// and deserializable responses.
pub use modules::responses::*;
