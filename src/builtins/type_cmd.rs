use super::Builtin;
use std::io;
use std::env;
use std::path::{Path, PathBuf};
use std::fs;
use std::os::unix::fs::PermissionsExt;

pub struct Type;

impl Builtin for Type {
    fn execute(&self, args: &[String], stdout: &mut dyn io::Write) -> Result<i32, String> {
        let mut exit_code = 0;
        
        for arg in args {
             match arg.as_str() {
                 "cd" | "echo" | "exit" | "pwd" | "true" | "false" | "export" | "alias" | "unalias" | "type" => {
                     writeln!(stdout, "{} is a shell builtin", arg).map_err(|e| e.to_string())?;
                 },
                 _ => {
                     if let Some(path) = find_executable(arg) {
                         writeln!(stdout, "{} is {}", arg, path.display()).map_err(|e| e.to_string())?;
                     } else {
                         writeln!(stdout, "42sh: type: {}: not found", arg).map_err(|e| e.to_string())?;
                         exit_code = 1;
                     }
                 }
             }
        }
        Ok(exit_code)
    }
}

fn find_executable(name: &str) -> Option<PathBuf> {
    if name.contains('/') {
        let path = PathBuf::from(name);
        if is_executable(&path) {
            return Some(path);
        }
        return None;
    }

    let path_env = env::var("PATH").unwrap_or_default();
    for path_str in path_env.split(':') {
        let path = Path::new(path_str).join(name);
        if is_executable(&path) {
            return Some(path);
        }
    }
    None
}

fn is_executable(path: &Path) -> bool {
    if let Ok(metadata) = fs::metadata(path) {
        if metadata.is_file() {
            return metadata.permissions().mode() & 0o111 != 0;
        }
    }
    false
}
