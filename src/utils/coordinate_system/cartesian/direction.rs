use std::fmt;
use anyhow::{Error, anyhow};
use super::axis::Axis;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction { 
    N, E, S, W,
    NE, NW, SE, SW
}

impl TryFrom<&str> for Direction {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "N" => Ok(Direction::N),
            "NE" => Ok(Direction::NE),
            "E" => Ok(Direction::E),
            "SE" => Ok(Direction::SE),
            "S" => Ok(Direction::S),
            "SW" => Ok(Direction::SW),
            "W" => Ok(Direction::W),
            "NW" => Ok(Direction::NW),
            _ => Err(anyhow!["Invalid value for direction `{}`", value])
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Direction::N => "N",
            Direction::NE => "NE",
            Direction::E => "E",
            Direction::SE => "SE",
            Direction::S => "S",
            Direction::SW => "SW",
            Direction::W => "W",
            Direction::NW => "NW"
        };
        write!(f, "{}", s)
    }
}

pub const TOUCHING_DIRECTIONS: [Direction; 4] = [
    Direction::N, Direction::E, Direction::S, Direction::W
];

pub const DIRECTIONS: [Direction; 8]  = [
    Direction::N, Direction::NE, Direction::E, Direction::SE,
    Direction::S, Direction::SW, Direction::W, Direction::NW
];

impl Direction {
    pub fn affected_axes(&self) -> Vec<Axis> {        
        match self {
            Direction::N | Direction::S => vec![Axis::Y],
            Direction::W | Direction::E => vec![Axis::X],
            _ => vec![Axis::X, Axis::Y]
        }
    }

    pub fn rotate(&self, degrees: isize) -> Self {
        if degrees % 45 != 0 {
            panic!("Invalid value for rotation degrees ({}). Degrees need to be in 45 steps (divisible by 45)", degrees);
        }
        let rotations: isize = degrees / 45;
        let index = DIRECTIONS.iter().position(|d| d == self).unwrap() as isize;
        let new_index = (index + rotations).rem_euclid(DIRECTIONS.len() as isize) as usize;
        DIRECTIONS[new_index]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction_rotation() {
        use Direction::*;

        assert_eq!(N.rotate(90), E);
        assert_eq!(N.rotate(-90), W);
        assert_eq!(N.rotate(180), S);
        assert_eq!(N.rotate(-180), S);
        assert_eq!(N.rotate(-45), NW);
        assert_eq!(N.rotate(45), NE);
        assert_eq!(N.rotate(135), SE);
        assert_eq!(N.rotate(-135), SW);
        assert_eq!(N.rotate(360), N);

        assert_eq!(S.rotate(90), W);
        assert_eq!(S.rotate(-90), E);

    }
}