use rustpython_parser::ast::{Located, Location};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Range {
    pub location: Location,
    pub end_location: Location,
}

impl Range {
    pub const fn new(location: Location, end_location: Location) -> Self {
        Self {
            location,
            end_location,
        }
    }

    pub fn from_located<T, U>(located: &Located<T, U>) -> Self {
        Range::new(located.location, located.end_location.unwrap())
    }

    pub fn contains(&self, other: &Range) -> bool {
        self.location <= other.location && self.end_location >= other.end_location
    }
}
