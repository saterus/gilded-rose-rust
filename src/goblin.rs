// This module cannot be changed. Other users depend ont the Item struct, so making any changes to
// it is not modifiable in the short term.
// -- The Management

/// TODO: add documentation here
/// -- The Management
#[derive(Debug)]
pub struct Item {
    pub name: &'static str,
    pub sell_in: i64,
    pub quality: i64,
}
