use super::Builtin;
use std::io;
use which::which;

pub struct Type;
impl Builtin for Type {
    fn execute(&self, args: &[String], stdout: &mut dyn io::Write) -> Result<i32, String> {
        // Basic type implementation
        for arg in args {
             match arg.as_str() {
                 "cd" | "echo" | "exit" | "pwd" | "true" | "false" | "export" | "alias" | "unalias" | "type" => {
                     writeln!(stdout, "{} is a shell builtin", arg).map_err(|e| e.to_string())?;
                 },
                 _ => {
                     // Check path
                     if let Ok(path) = which(arg) {
                         writeln!(stdout, "{} is {}", arg, path.display()).map_err(|e| e.to_string())?;
                     } else {
                         writeln!(stdout, "bash: type: {}: not found", arg).map_err(|e| e.to_string())?;
                         return Ok(1);
                     }
                 }
             }
        }
        Ok(0)
    }
}
