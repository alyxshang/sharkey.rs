/*
Sharkey.rs by Alyx Shang.
Licensed under the FSL v1.
*/

/// A structure
/// to bridge the gap
/// between the "Response"
/// structure from the "reqwest"
/// module and other types.
pub struct Bridge{
    pub body: Option<String>
}
