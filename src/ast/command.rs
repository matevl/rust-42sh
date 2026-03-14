use super::CommandList;
use super::redirection::Redirection;

#[derive(Debug, Clone, PartialEq)]
pub struct SimpleCommand {
    pub assignments: Vec<String>, // var=value
    pub words: Vec<String>,       // command and arguments
    pub redirections: Vec<Redirection>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfCommand {
    pub condition: Box<CommandList>,
    pub then_branch: Box<CommandList>,
    pub elif_branches: Vec<(CommandList, CommandList)>, // (condition, body)
    pub else_branch: Option<Box<CommandList>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Simple(SimpleCommand),
    If(IfCommand),
    // Future additions: Pipe, While, For, Case, Subshell, etc.
}
