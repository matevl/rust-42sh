use super::Builtin;
use std::io;
use std::env;

pub struct Cd;
impl Builtin for Cd {
    fn execute(&self, args: &[String], _stdout: &mut dyn io::Write) -> Result<i32, String> {
        let path = args.get(0).map(|s| s.as_str()).unwrap_or(""); 
        let root = env::var("HOME").unwrap_or_else(|_| "/".to_string());
        let target = if path.is_empty() { &root } else { path };
        
        match env::set_current_dir(target) {
            Ok(_) => Ok(0),
            Err(e) => Err(format!("cd: {}: {}", target, e)),
        }
    }
}
