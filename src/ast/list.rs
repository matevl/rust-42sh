use super::command::Command;

#[derive(Debug, Clone, PartialEq)]
pub struct CommandList {
    pub commands: Vec<Command>,
    pub asynchronous: bool,
}
