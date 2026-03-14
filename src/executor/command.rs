use crate::ast::{Command, CommandList, IfCommand, SimpleCommand};
use crate::builtins::{find_builtin};
use crate::executor::redirection::RedirectionManager;
use std::process::{Command as StdCommand};
use std::os::unix::process::CommandExt;
use std::io::{self};

pub struct Executor;

impl Executor {
    pub fn new() -> Self {
        Executor
    }

    pub fn execute(&self, command_list: &CommandList) -> Result<i32, String> {
        let mut last_status = 0;
        for cmd in &command_list.commands {
            last_status = self.execute_command(cmd)?;
        }
        Ok(last_status)
    }

    fn execute_command(&self, cmd: &Command) -> Result<i32, String> {
        match cmd {
            Command::Simple(simple) => self.execute_simple(simple),
            Command::If(if_cmd) => self.execute_if(if_cmd),
        }
    }

    fn execute_simple(&self, simple: &SimpleCommand) -> Result<i32, String> {
        // If there are no words, handle assignments/redirections alone
        if simple.words.is_empty() {
             // e.g. `>file` creates the file
             let mut rm = RedirectionManager::new();
             if let Err(e) = rm.apply(&simple.redirections) {
                 return Err(format!("Redirection error: {}", e));
             }
             // rm drops here, restoring FDs (closing file if opened)
             return Ok(0);
        }

        let program = &simple.words[0];
        let args = &simple.words[1..];

        // Builtin
        if let Some(builtin) = find_builtin(program) {
            let mut rm = RedirectionManager::new();
            if let Err(e) = rm.apply(&simple.redirections) {
                return Err(format!("Redirection error: {}", e));
            }
            // Execute builtin
            // Note: builtins write to stdout/stderr which are now redirected FDs
            let result = builtin.execute(args, &mut io::stdout());
            
            // rm drops here automatically, restoring FDs for the shell process
            return result;
        }

        // External Command
        let mut child = StdCommand::new(program);
        child.args(args);
        
        let redirections = simple.redirections.clone();
        
        // We use pre_exec to setup redirections in the child process
        unsafe {
            child.pre_exec(move || {
                let mut rm = RedirectionManager::new();
                if let Err(e) = rm.apply(&redirections) {
                    return Err(io::Error::new(io::ErrorKind::Other, e));
                }
                // Important: Don't restore FDs! The child process needs them.
                rm.keep(); 
                Ok(())
            });
        }

        match child.status() {
            Ok(status) => Ok(status.code().unwrap_or(0)),
            Err(e) => Err(format!("Failed to execute '{}': {}", program, e)),
        }
    }

    fn execute_if(&self, if_cmd: &IfCommand) -> Result<i32, String> {
        let status = self.execute(&if_cmd.condition)?;
        if status == 0 {
            self.execute(&if_cmd.then_branch)
        } else {
            for (cond, body) in &if_cmd.elif_branches {
                 let elif_status = self.execute(cond)?;
                 if elif_status == 0 {
                     return self.execute(body);
                 }
            }
            if let Some(else_branch) = &if_cmd.else_branch {
                self.execute(else_branch)
            } else {
                Ok(0)
            }
        }
    }
}
