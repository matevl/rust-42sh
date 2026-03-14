use std::io;

mod cd;
mod echo;
mod exit;
mod pwd;
mod true_false;
mod export;
mod alias;
mod type_cmd;

pub trait Builtin {
    fn execute(&self, args: &[String], stdout: &mut dyn io::Write) -> Result<i32, String>;
}

pub fn find_builtin(name: &str) -> Option<Box<dyn Builtin>> {
    match name {
        "cd" => Some(Box::new(cd::Cd)),
        "echo" => Some(Box::new(echo::Echo)),
        "exit" => Some(Box::new(exit::Exit)),
        "pwd" => Some(Box::new(pwd::Pwd)),
        "true" => Some(Box::new(true_false::True)),
        "false" => Some(Box::new(true_false::False)),
        "export" => Some(Box::new(export::Export)),
        "alias" => Some(Box::new(alias::Alias)),
        "unalias" => Some(Box::new(alias::Unalias)),
        "type" => Some(Box::new(type_cmd::Type)),
        _ => None,
    }
}
