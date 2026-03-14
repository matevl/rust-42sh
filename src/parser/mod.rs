use crate::ast::{Command, CommandList, IfCommand, Redirection, RedirectionType, SimpleCommand};
use crate::lexer::{Lexer, Token};

/// The Parser struct, which takes a generic Lexer.
pub struct Parser<I: Iterator<Item = char>> {
    lexer: Lexer<I>,
    current_token: Token,
}

impl<I: Iterator<Item = char>> Parser<I> {
    pub fn new(mut lexer: Lexer<I>) -> Self {
        let current_token = lexer.next_token();
        Parser {
            lexer,
            current_token,
        }
    }

    /// Advances to the next token.
    fn next_token(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    /// Parses a list of commands (e.g. separated by newlines or semicolons).
    /// Used for scripts or when reading multiple commands at once.
    pub fn parse_command_list(&mut self) -> Result<CommandList, String> {
        let mut commands = Vec::new();

        loop {
            // Consume delimiters
            while matches!(self.current_token, Token::NewLine | Token::Semi) {
                self.next_token();
            }

            if self.current_token == Token::EOF {
                break;
            }

            // Reserved words that terminate a command list context
            if matches!(self.current_token, Token::Fi | Token::Then | Token::Else | Token::Elif) {
                 break;
            }

            let command = self.parse_command()?;
            commands.push(command);
        }

        Ok(CommandList {
            commands,
            asynchronous: false,
        })
    }

    /// Parses a single command, returning immediately.
    /// Useful for REPL to execute commands as they are typed.
    pub fn parse_next_command(&mut self) -> Result<Option<Command>, String> {
        // Consume delimiters
        while matches!(self.current_token, Token::NewLine | Token::Semi) {
            self.next_token();
        }

        if self.current_token == Token::EOF {
            return Ok(None);
        }

        let cmd = self.parse_command()?;
        Ok(Some(cmd))
    }

    fn parse_command(&mut self) -> Result<Command, String> {
        match self.current_token {
            Token::If => self.parse_if(),
            Token::Word(_) | Token::IoNumber(_) | Token::Less | Token::Great | Token::DGreat | Token::DLess | Token::LessGreat | Token::LessAnd | Token::GreatAnd | Token::CLobber => Ok(Command::Simple(self.parse_simple_command()?)),
            _ => Err(format!("Unexpected token: {:?}", self.current_token)),
        }
    }

    fn parse_if(&mut self) -> Result<Command, String> {
        self.next_token(); // eat 'if'
        let condition = self.parse_command_list()?;
        
        if self.current_token != Token::Then {
            return Err("Expected 'then'".to_string());
        }
        self.next_token(); // eat 'then'

        let then_branch = self.parse_command_list()?;
        
        let mut elif_branches = Vec::new();
        while self.current_token == Token::Elif {
            self.next_token(); 
            let cond = self.parse_command_list()?;
            if self.current_token != Token::Then {
                return Err("Expected 'then' after elif condition".to_string());
            }
            self.next_token(); 
            let body = self.parse_command_list()?;
            elif_branches.push((cond, body));
        }

        let else_branch = if self.current_token == Token::Else {
            self.next_token(); 
            Some(Box::new(self.parse_command_list()?))
        } else {
            None
        };

        if self.current_token != Token::Fi {
             return Err(format!("Expected 'fi', found {:?}", self.current_token));
        }
        self.next_token(); // eat 'fi'

        Ok(Command::If(IfCommand {
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            elif_branches,
            else_branch,
        }))
    }

    fn parse_simple_command(&mut self) -> Result<SimpleCommand, String> {
        let mut words = Vec::new();
        let mut assignments = Vec::new();
        let mut redirections = Vec::new();

        loop {
            match self.current_token.clone() {
                Token::Word(w) => {
                    if w.contains('=') && words.is_empty() {
                         assignments.push(w);
                    } else {
                         words.push(w);
                    }
                    self.next_token();
                }
                Token::IoNumber(s) => {
                     let fd = s.parse::<u32>().ok();
                     self.next_token();
                     self.parse_redirection_body(fd, &mut redirections)?;
                }
                Token::Less | Token::Great | Token::DGreat | Token::DLess | Token::LessGreat | Token::LessAnd | Token::GreatAnd | Token::CLobber => {
                     self.parse_redirection_body(None, &mut redirections)?;
                }
                _ => break, 
            }
        }
        
        Ok(SimpleCommand {
            assignments,
            words,
            redirections,
        })
    }

    fn parse_redirection_body(&mut self, fd: Option<u32>, redirections: &mut Vec<Redirection>) -> Result<(), String> {
         let token = self.current_token.clone();
         match token {
             Token::Less | Token::Great | Token::DGreat | Token::DLess | Token::LessGreat | Token::LessAnd | Token::GreatAnd | Token::CLobber => {
                 self.next_token();
                 if let Token::Word(target) = &self.current_token {
                     let op = match token {
                         Token::Less => RedirectionType::Input,
                         Token::Great => RedirectionType::Output,
                         Token::DGreat => RedirectionType::Append, 
                         Token::DLess => RedirectionType::HereDoc,
                         Token::LessGreat => RedirectionType::ReadWrite,
                         Token::LessAnd => RedirectionType::DupInput,
                         Token::GreatAnd => RedirectionType::DupOutput,
                         Token::CLobber => RedirectionType::CLobber, 
                         _ => unreachable!(),
                     };
                     redirections.push(Redirection {
                         fd,
                         redirection_type: op,
                         target: target.clone(),
                     });
                     self.next_token();
                     Ok(())
                 } else {
                     Err("Expected filename after redirection".to_string())
                 }
             }
             _ => Err(format!("Expected redirection operator, found {:?}", token)),
         }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::ast::Command;

    #[test]
    fn test_simple_command_parsing() {
        let input = "ls -la";
        let lexer = Lexer::new(input.chars());
        let mut parser = Parser::new(lexer);
        
        let result = parser.parse_command_list().unwrap();
        assert_eq!(result.commands.len(), 1);
        
        if let Command::Simple(cmd) = &result.commands[0] {
             assert_eq!(cmd.words, vec!["ls", "-la"]);
        } else {
            panic!("Expected SimpleCommand");
        }
    }
}
