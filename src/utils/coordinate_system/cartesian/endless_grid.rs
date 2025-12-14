use std::fmt;

use funty::Integral;
use num::{Num, ToPrimitive};

use super::coord::Coord;

#[derive(Clone)]
pub struct Grid<V> {
    pub coordinates: Vec<Coord<V>>,
    pub min_x: V,
    pub max_x: V,
    pub min_y: V,
    pub max_y: V,
    pub height: V,
    pub width: V

}

impl<V> Grid<V>
where V: Integral
{
    pub fn new(coordinates: Vec<Coord<V>>) -> Self
    {
        let mut min_x = V::MAX;
        let mut max_x = V::MIN;
        let mut min_y = V::MAX;
        let mut max_y = V::MIN;
        for coord in &coordinates {
            if coord.x > max_x {
                max_x = coord.x;                
            } else if coord.x < min_x {
                min_x = coord.x;
            }

            if coord.y > max_y {
                max_y = coord.y;
            } else if coord.y < min_y {
                min_y = coord.y;
            }
        }
        let height = max_y - min_y;
        let width = max_x - min_x;

        Self { coordinates, min_x, max_x, min_y, max_y, height, width }
    }
}

impl<V> fmt::Display for Grid<V>
where 
    V: fmt::Display + std::cmp::PartialOrd + Num + Clone + Copy + ToPrimitive
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n")?;
        let mut y = self.min_y;
        while y <= self.max_y {
            let mut x = self.min_x;
            let mut line: String = String::with_capacity(num::ToPrimitive::to_usize(&self.width).unwrap());
            while x <= self.max_x {
                let c: Coord<V> = Coord::new(x.clone(), y.clone());
                if self.coordinates.contains(&c) {
                    line.push('×');
                } else {
                    line.push(' ');
                }
                x = x + num::one();
            }
            write!(f, "{}\n", line)?;
            y = y + num::one();
        }
        write!(f, "\n")
    }
}