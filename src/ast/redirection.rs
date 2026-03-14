#[derive(Debug, Clone, PartialEq)]
pub enum RedirectionType {
    Input,          // <
    Output,         // >
    Append,         // >>
    ReadWrite,      // <>
    DupInput,       // <&
    DupOutput,      // >&
    HereDoc,        // <<
    CLobber,        // >|
}

#[derive(Debug, Clone, PartialEq)]
pub struct Redirection {
    pub fd: Option<u32>, // File descriptor (e.g., "2" in "2>")
    pub redirection_type: RedirectionType,
    pub target: String,  // Filename or fd target
}
