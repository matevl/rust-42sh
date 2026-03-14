use super::Builtin;
use std::io;
use std::process;

pub struct Exit;
impl Builtin for Exit {
    fn execute(&self, args: &[String], _stdout: &mut dyn io::Write) -> Result<i32, String> {
        let code = if let Some(arg) = args.get(0) {
            arg.parse::<i32>().unwrap_or(0)
        } else {
            0
        };
        process::exit(code);
    }
}
