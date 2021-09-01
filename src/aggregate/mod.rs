use std::fmt;
use std::error::Error;

pub trait Aggregate
where
    Self::Items: Iterator<Item=Self::Item>
{
    type Item;
    type Items;
    fn aggregate(&self) -> Result<Self::Items, AggregateError>;
}

#[derive(Debug)]
pub struct AggregateError {
    pub msg: &'static str 
}

impl fmt::Display for AggregateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "notice error: {}", self.msg)
    }
}

impl Error for AggregateError {}