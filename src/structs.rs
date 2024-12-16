/// WordSmith modules take in the file, perform
/// their analysis and output a Vec of Message.
pub trait Module {
    fn check(filepath: &str) -> Vec<Message>;
}

pub enum MessageType {
    WeaselWord,
}

/// Message is returned by a Module that indicates the
/// offending line number and a short description of 
/// what to do instead.
pub struct Message {
    pub message_type: MessageType,
    pub line_no: u32,
    pub message: String,
}