use super::Builtin;
use std::io;

pub struct True;
impl Builtin for True {
    fn execute(&self, _args: &[String], _stdout: &mut dyn io::Write) -> Result<i32, String> {
        Ok(0)
    }
}

pub struct False;
impl Builtin for False {
    fn execute(&self, _args: &[String], _stdout: &mut dyn io::Write) -> Result<i32, String> {
        Ok(1)
    }
}
