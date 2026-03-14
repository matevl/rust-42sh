use super::Builtin;
use std::io;

pub struct Echo;
impl Builtin for Echo {
    fn execute(&self, args: &[String], stdout: &mut dyn io::Write) -> Result<i32, String> {
        let mut no_newline = false;
        let mut start_index = 0;

        if let Some(first) = args.get(0) {
            if first == "-n" {
                no_newline = true;
                start_index = 1;
            }
        }

        let content = args[start_index..].join(" ");
        write!(stdout, "{}", content).map_err(|e| e.to_string())?;
        if !no_newline {
            writeln!(stdout).map_err(|e| e.to_string())?;
        }
        
        Ok(0)
    }
}
