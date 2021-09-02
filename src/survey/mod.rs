use std::fmt;
use std::error::Error;

pub trait Survey
where
    Self::Item: Weightable,
    Self::Items: Iterator<Item=Self::Item>
{
    type Item;
    type Items;
    fn gather(&self) -> Result<Self::Items, SurveyError>;
}

pub trait Weightable 
where
    Self::Weight: Ord
{
    type Weight;
    fn weight(&self) -> Option<Self::Weight>;
}

pub fn survey_weight<S: Survey>(survey: &S) -> Result<impl Iterator<Item=(S::Item, <S::Item as Weightable>::Weight)>, SurveyError> {
    let items = survey.gather()?;
    let weighted = items.filter_map(|item| {
        let weight = item.weight()?;
        Some((item, weight))
    });
    Ok(weighted)
}

#[derive(Debug)]
pub struct SurveyError {
    pub msg: &'static str 
}

impl fmt::Display for SurveyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "notice error: {}", self.msg)
    }
}

impl Error for SurveyError {}
