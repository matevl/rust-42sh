use super::Builtin;
use std::io;

pub struct Alias;
impl Builtin for Alias {
    fn execute(&self, _args: &[String], _stdout: &mut dyn io::Write) -> Result<i32, String> {
        // Placeholder for alias implementation
        Ok(0)
    }
}

pub struct Unalias;
impl Builtin for Unalias {
    fn execute(&self, _args: &[String], _stdout: &mut dyn io::Write) -> Result<i32, String> {
        // Placeholder for unalias implementation
        Ok(0)
    }
}
