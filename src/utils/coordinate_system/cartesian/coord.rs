use std::fmt;
use std::cmp::Ordering;
use std::ops::{Add, Sub, AddAssign, SubAssign};
use funty::Signed;

use super::axis::Axis;
use super::direction::Direction;


#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord<T> {
    pub x: T,
    pub y: T
}

impl<T> Coord<T> {
    pub fn new(x: T, y: T) -> Self {
        Coord { x, y }
    }

    pub fn get<'a>(&'a self, axis: &Axis) -> &'a T {
        match axis {
            Axis::X => &self.x,
            Axis::Y => &self.y
        }
    }

    pub fn get_mut<'a>(&'a mut self, axis: &Axis) -> &'a mut T {
        match axis {
            Axis::X => &mut self.x,
            Axis::Y => &mut self.y
        }
    }
}

impl<T> Coord<T> 
where
    T: Signed
{
    pub fn manhattan_distance(&self, other: &Self) -> T {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }    
}

impl<T> Coord<T>
where
    T: num::Integer + Copy
{
    pub fn get_neighbour(&self, direction: &Direction) -> Self {
        match direction {
            Direction::N => Coord::new(self.x, self.y + num::one()),
            Direction::NE => Coord::new(self.x + num::one(), self.y + num::one()),
            Direction::E => Coord::new(self.x + num::one(), self.y),
            Direction::SE => Coord::new(self.x + num::one(), self.y - num::one()),
            Direction::S => Coord::new(self.x, self.y - num::one()),
            Direction::SW => Coord::new(self.x - num::one(), self.y - num::one()),
            Direction::W => Coord::new(self.x - num::one(), self.y),
            Direction::NW => Coord::new(self.x - num::one(), self.y + num::one()),
        }
    }
}

impl<T: Add<Output = T>> Add for Coord<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Coord::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: AddAssign> AddAssign for Coord<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y
    }
}

impl<T: SubAssign> SubAssign for Coord<T>
where 
    T: num::Signed
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: Sub<Output = T>> Sub for Coord<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Coord::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T> fmt::Display for Coord<T> 
where T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T> fmt::Debug for Coord<T> 
where T: fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "C({:?}, {:?})", self.x, self.y)
    }
}

impl<T: Ord> Ord for Coord<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        let y_cmp = self.y.cmp(&other.y);
        match y_cmp {
            Ordering::Equal => {
                self.x.cmp(&other.x)
            },
            _ => y_cmp
        }
    }
}

impl<T: PartialOrd + Ord> PartialOrd for Coord<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> From<&Direction> for Coord<T>
where 
    T: num::Signed
{
    fn from(direction: &Direction) -> Self {
        match direction {
            Direction::N  => Coord::new( T::zero(), T::one()), 
            Direction::E  => Coord::new( T::one(),  T::zero()),
            Direction::S  => Coord::new( T::zero(),-T::one()),
            Direction::W  => Coord::new(-T::one(),  T::zero()),
            Direction::NE => Coord::new( T::one(),  T::one()),
            Direction::NW => Coord::new(-T::one(),  T::one()),
            Direction::SE => Coord::new( T::one(), -T::one()),
            Direction::SW => Coord::new(-T::one(), -T::one()),
        }
    }
}

impl<T> From<(T, T)> for Coord<T>
{
    fn from(tuple: (T, T)) -> Self {
        Coord::new( tuple.0, tuple.1 )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::direction::DIRECTIONS;

    #[test]
    fn test_coord_neighbours() {
        let c = Coord::new(0, 0);
        let neighbours: Vec<_> = DIRECTIONS.iter().map(|dir| {
            c.get_neighbour(dir)
        }).collect();  
        assert_eq!(neighbours, vec![
            Coord::new(0, 1),
            Coord::new(1, 1),
            Coord::new(1, 0),
            Coord::new(1, -1),
            Coord::new(0, -1),
            Coord::new(-1, -1),
            Coord::new(-1, 0),
            Coord::new(-1, 1),
        ])
    }
}