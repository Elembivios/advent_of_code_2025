use std::fmt;

use super::coord::Coord;
use super::point::Point;
use super::direction::{Direction, DIRECTIONS, TOUCHING_DIRECTIONS};
use super::axis::Axis;



#[derive(Clone)]
pub struct Grid<V> {
    pub map: Vec<V>,
    pub height: usize,
    pub width: usize
}

impl<V> Grid<V>
{
    pub fn new(map: Vec<Vec<V>>) -> Self {
        let height = map.len();
        let width = map.get(0).unwrap_or(&vec![]).len();
        for (i, row) in map.iter().enumerate() {
            if row.len() != width {
                panic!("Row {} width {} is not the same as all the rest ({})", i, row.len(), width);
            }
        }
        Grid {
            map: map.into_iter().flatten().collect(),
            height, width
        }
    }

    pub fn contains<T>(&self, coord: &Coord<T>) -> bool
    where T: num::Integer + Copy + num::FromPrimitive
    {   
        let h = num::FromPrimitive::from_usize(self.height).unwrap();
        let w = num::FromPrimitive::from_usize(self.width).unwrap();        
        if coord.y >= num::zero() && coord.y < h && coord.x >= num::zero() && coord.x < w {
            true
        } else {
            false
        }
    }

    pub fn get_neighbour(&self, coord: &Coord<usize>, direction: &Direction) -> Option<Coord<usize>> {
        match direction {
            Direction::N => if coord.y == 0 { None } else { Some(Coord::new(coord.x, coord.y - 1)) },            
            Direction::E => if coord.x == self.width - 1 { None } else { Some(Coord::new(coord.x + 1, coord.y)) },
            Direction::S => if coord.y == self.height - 1 { None } else { Some(Coord::new(coord.x, coord.y + 1)) },
            Direction::W => if coord.x == 0 { None } else { Some(Coord::new(coord.x - 1, coord.y)) },
            Direction::NE => if coord.x == self.width - 1 || coord.y == 0 { None } else { Some(Coord::new(coord.x + 1, coord.y - 1))},
            Direction::SE => if coord.x == self.width - 1 || coord.y == self.height - 1 { None } else { Some(Coord::new(coord.x + 1, coord.y + 1))},
            Direction::SW => if coord.x == 0 || coord.y == self.height - 1 { None } else { Some(Coord::new(coord.x - 1, coord.y + 1))},
            Direction::NW => if coord.x == 0 || coord.y == 0 { None } else { Some(Coord::new(coord.x - 1, coord.y - 1))},
        }
    }

    pub fn get_index(&self, coord: &Coord<usize>) -> usize {
        coord.y * self.width + coord.x
    }

    pub fn get_val(&self, coord: &Coord<usize>) -> &V {
        &self.map[self.get_index(coord)]
    }

    pub fn get_val_mut(&mut self, coord: &Coord<usize>) -> &mut V {
        &mut self.map[coord.y * self.width + coord.x]
    }

    pub fn get_point(&self, coord: &Coord<usize>) -> Point<usize, &V> {
        Point::from_coord(coord.clone(), self.get_val(coord))
    }

    pub fn get_point_mut(&mut self, coord: &Coord<usize>) -> Point<usize, &mut V> {
        Point::from_coord(coord.clone(), self.get_val_mut(coord))
    }

    pub fn neighbour_coords(&self, coord: &Coord<usize>) -> Vec<Coord<usize>> {
        TOUCHING_DIRECTIONS
            .iter()
            .filter_map(|direction| {
                self.get_neighbour(coord, direction)
            }).collect()
    }

    pub fn neigbour_coords_optional(&self, coord: &Coord<usize>) -> Vec<Option<Coord<usize>>> {
        TOUCHING_DIRECTIONS
            .iter()
            .map(|direction| {
                self.get_neighbour(coord, direction)
            }).collect()
    }

    /// Get's neighbour coords of a specified coordinate. If the neighbour
    /// coordinate is off the edge of map, it returns the one on the opposite 
    /// end of map.
    pub fn neighbour_coords_wrapping(&self, coord: &Coord<usize>) -> Vec<Coord<usize>> {
        let mut neighbours = vec![];
        // North
        if coord.y == 0 {
            neighbours.push(Coord::new(coord.x, self.height - 1));
        } else {
            neighbours.push(Coord::new(coord.x, coord.y - 1));
        }
        // East
        if coord.x == self.width - 1 {
            neighbours.push(Coord::new(0, coord.y));
        } else {
            neighbours.push(Coord::new(coord.x + 1, coord.y));
        }
        // South
        if coord.y == self.height - 1 {
            neighbours.push(Coord::new(coord.x, 0));
        } else {
            neighbours.push(Coord::new(coord.x, coord.y + 1));
        }

        // West
        if coord.x == 0 {
            neighbours.push(Coord::new(self.width - 1, coord.y));
        } else {
            neighbours.push(Coord::new(coord.x - 1, coord.y));
        }        
        neighbours
    }

    pub fn adjacent_coords(&self, coord: &Coord<usize>) -> Vec<Coord<usize>> {
        DIRECTIONS
            .iter()
            .filter_map(|direction| {
                self.get_neighbour(coord, direction)
            }).collect()
    }

    pub fn iter_values(&self) -> impl Iterator<Item=&V> {
        self.map.iter()
    }

    pub fn iter_coords(&self) -> impl Iterator<Item=Coord<usize>> + '_ {
        (0..self.height).into_iter().map(move |y| (0..self.width).into_iter().map(move |x| Coord::new(x, y))).flatten()
    }

    // pub fn iter_coords_mut(&mut self) -> impl Iterator<Item=
    

    pub fn iter_points(&self) -> impl Iterator<Item=Point<usize, &V>> + '_ {
        self.iter_coords().map(|c| {
            let val = self.get_val(&c);
            Point::from_coord(c, val)
        })
    }
    
    pub fn iter_points_mut(&mut self) -> impl Iterator<Item=Point<usize, &mut V>> {
        self.map
            .iter_mut()
            .enumerate()
            .map(|(i, v)| {
                let y = i / self.width;
                let x = i % self.width;
                let c = Coord::new(x, y);
                Point::from_coord(c, v)
            })
    }

    pub fn direction_iter(&self, direction: Direction, current_coord: Coord<usize>) -> GridDirectionIterator {
        let axis = direction.affected_axes()[0];
        GridDirectionIterator {
            height: self.height,
            width: self.width,
            direction, current_coord, axis
        }
    }

    pub fn wrapped_direction_iter(&self, direction: Direction, current_coord: Coord<usize>) -> GridWrappedDirectionIterator {
        let axis = direction.affected_axes()[0];
        GridWrappedDirectionIterator { 
            height: self.height, 
            width: self.width, 
            direction, current_coord, axis 
        }
    }
}

impl<V> Grid<V>
where V: Copy
 {
    pub fn rotate(&mut self, clockwise: bool) {
        let new_map: Vec<V> = if clockwise {
            (0..self.width).map(|x| {
                (0..self.height).rev().map(|y| {
                    self.map[y * self.width + x]
                }).collect::<Vec<V>>()
            }).flatten().collect()
        } else {
            (0..self.width).rev().map(|x| {
                (0..self.height).map(|y| {
                    self.map[y * self.width + x]
                }).collect::<Vec<V>>()
            }).flatten().collect()
        };
        self.map = new_map;
        let width = self.width;
        self.width = self.height;
        self.height = width;        
    }
}


impl<V> Grid<V> 
where 
    V: fmt::Display
{
    pub fn display_with_points(&self, points: Vec<Coord<usize>>, display_char: char) {
        print!("\n");
        for y in 0..self.height {
            for x in 0..self.width {
                let current = Coord::new(x, y);
                if points.contains(&current) {
                    print!("{}", display_char);
                } else {
                    let p = self.get_point(&current);
                    print!("{}", p);
                }
            }
            print!("\n");
        }
    }
}

impl<V> fmt::Display for Grid<V> 
where
    V: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n")?;
        for y in 0..self.height {            
            for x in 0..self.width {
                let v = self.get_val(&Coord::new(x, y));
                    write!(f, "{}", v)?;
            }
            write!(f, "\n")?;
        }
        write!(f, "\n")
    }
}



// Endles iterator of the grid in specified direction
// When it gets to the edge it jumps to the other side and 
// continues iterating in that direction.
pub struct GridDirectionIterator {
    height: usize,
    width: usize,
    direction: Direction,
    current_coord: Coord<usize>,
    axis: Axis,
}

impl Iterator for GridDirectionIterator {
    type Item = Coord<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.current_coord.get_mut(&self.axis);
        match self.direction {
            Direction::S => {
                if *val == self.height - 1 { return None; } else { *val += 1; }
            }
            Direction::E => {
                if *val == self.width - 1 { return None; } else { *val += 1; }
            },
            Direction::N => {
                if *val == 0 { return None; } else { *val -= 1; }
            }
            Direction::W => {
                if *val == 0 { return None; } else { *val -= 1; }
            },
            _ => unimplemented!("Iterator for direction {:?} is not implemented.", self.direction)
        }        
        Some(self.current_coord.clone())
    }
}

pub struct GridWrappedDirectionIterator {
    height: usize,
    width: usize,
    direction: Direction,
    current_coord: Coord<usize>,
    axis: Axis,
}

impl Iterator for GridWrappedDirectionIterator {
    type Item = Coord<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.current_coord.get_mut(&self.axis);
        match self.direction {
            Direction::S => {
                if *val == self.height - 1 { *val = 0; } else { *val += 1; }
            }
            Direction::E => {
                if *val == self.width - 1 { *val = 0; } else { *val += 1; }
            },
            Direction::N => {
                if *val == 0 { *val = self.height - 1; } else { *val -= 1; }
            }
            Direction::W => {
                if *val == 0 { *val = self.width - 1; } else { *val -= 1; }
            },
            _ => unimplemented!("Iterator for direction {:?} is not implemented.", self.direction)
        }        
        Some(self.current_coord.clone())
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    fn construct_grid() -> Grid<char> {
        let map = (0..10).into_iter().map(|_y| {
            (0..10).into_iter().map(|_x| {
                '.'
            }).collect()
        }).collect();
        Grid::new(map)
    }

     #[test]
    fn test_contains() {
        let grid = construct_grid();
        println!("{}", grid);
        let c = Coord::new(0, 0);
        assert_eq!(grid.contains(&c), true);
        let c = Coord::new(-1, -1);
        assert_eq!(grid.contains(&c), false);
        let c: Coord<usize> = Coord::new(1, 1);
        assert_eq!(grid.contains(&c), true);
    }
}