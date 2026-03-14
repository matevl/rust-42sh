use super::Builtin;
use std::io;
use std::env;

pub struct Export;
impl Builtin for Export {
    fn execute(&self, args: &[String], _stdout: &mut dyn io::Write) -> Result<i32, String> {
        for arg in args {
            if let Some((key, value)) = arg.split_once('=') {
                unsafe { env::set_var(key, value); }
            }
        }
        Ok(0)
    }
}
