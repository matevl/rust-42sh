use super::Builtin;
use std::io;
use std::env;

pub struct Pwd;
impl Builtin for Pwd {
    fn execute(&self, _args: &[String], stdout: &mut dyn io::Write) -> Result<i32, String> {
        match env::current_dir() {
            Ok(path) => {
                writeln!(stdout, "{}", path.display()).map_err(|e| e.to_string())?;
                Ok(0)
            }
            Err(e) => Err(format!("pwd: {}", e)),
        }
    }
}
