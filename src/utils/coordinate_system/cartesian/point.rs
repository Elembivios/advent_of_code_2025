use std::fmt;
use super::coord::Coord;

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub struct Point<T, U> {
    pub coord: Coord<T>,
    pub value: U
}

impl<T, U> Point<T, U> {
    pub fn new(x: T, y: T, value: U) -> Self {
        Point { coord: Coord::new(x, y), value }
    }

    pub fn from_coord(coord: Coord<T>, value: U) -> Self {
        Point { coord, value }
    }
}

impl<T, U> fmt::Display for Point<T, U> 
where
    U: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}