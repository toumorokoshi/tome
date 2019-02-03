use std::path::Path;
/// Any executable script
/// can be added to be executed, but
/// it's possible to add metadata
/// to the script via comments as well.
pub struct Script {
    /// determines if the script should
    /// be sourced or not.
    should_source: bool
}

impl Script {
    pub fn load(body: Path) -> Script {
    }
    pub fn execute(&self) -> {
    }
}