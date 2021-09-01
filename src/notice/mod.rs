use std::fmt;
use std::error::Error;

pub trait Notice {
    type Content;
    fn send(&self, content: Self::Content) -> Result<(), NoticeError>;
}

#[derive(Debug)]
pub struct NoticeError {
    pub msg: &'static str 
}

impl fmt::Display for NoticeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "notice error: {}", self.msg)
    }
}

impl Error for NoticeError {}