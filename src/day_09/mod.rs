use crate::utils::coordinate_system::cartesian::{Coord, Direction};

pub struct MovieTheater {
    red_tiles: Vec<Coord<usize>>
}

impl crate::Advent for MovieTheater {
    fn new(data: &str) -> Self
        where 
            Self: Sized {
        let red_tiles: Vec<Coord<usize>> = data.lines().map(|l| {
            let (lhs, rhs) = l.split_once(',').unwrap();
            Coord::new(lhs.parse::<usize>().unwrap(), rhs.parse::<usize>().unwrap())
        }).collect();

        Self { red_tiles }
    }

    fn part_01(&self) -> String {
        let mut max_area: usize = 0;
        for (i, lhs) in self.red_tiles.iter().enumerate() {
            for rhs in self.red_tiles.iter().skip(i + 1) {
                let area = Self::area(lhs, rhs);
                if area > max_area {
                    max_area = area
                }
                
            }
        }
        max_area.to_string()
    }

    fn part_02(&self) -> String {
        let mut path: Vec<Coord<usize>> = vec![];
        
        
        
        // let mut areas: Vec<(usize, usize, usize)> = vec![];
        // for (i, lhs) in self.red_tiles.iter().enumerate() {
        //     'search: for (j, rhs) in self.red_tiles.iter().enumerate().skip(i + 1) {
        //         let (min_x, max_x) = Self::minmax(lhs.x, rhs.x);
        //         let (min_y, max_y) = Self::minmax(lhs.y, rhs.y);

        //         let mut on_edge: Vec<(_, _, Direction)> = vec![];
        //         for (z, middle) in self.red_tiles.iter().enumerate() {
        //             if z == i || z == j {
        //                 continue;
        //             }

        //             if middle.x < min_x || middle.x > max_x || middle.y < min_y || middle.y > max_y {
        //                 continue;
        //             }

        //             if middle.x == min_x {
        //                 let came_from = if lhs.x == min_x { lhs } else { rhs };                        
        //                 on_edge.push((middle, came_from, Direction::W));
        //             } else if middle.x == max_x {
        //                 let came_from = if lhs.x == max_x { lhs } else { rhs };                        
        //                 on_edge.push((middle, came_from, Direction::E));
        //             } else if middle.y == min_y {
        //                 let came_from = if lhs.y == min_y { lhs } else { rhs };                        
        //                 on_edge.push((middle, came_from, Direction::N));
        //             } else if middle.y == max_y {
        //                 let came_from = if lhs.y == max_y { lhs } else { rhs };                        
        //                 on_edge.push((middle, came_from, Direction::S));                                
        //             } else {
        //                 println!("In middle. {} -> {} | {}", lhs, rhs, middle);
        //                 continue 'search;   
        //             }
        //         }

        //         for (middle, came_from, edge) in on_edge {
        //             let next = self.red_tiles.iter()
        //                 .filter(|other| other.x == middle.x || other.y == middle.y)
        //                 .filter(|other| !(*other == came_from || *other == middle))
        //                 .next().unwrap();
        //             let ok = match edge {
        //                 Direction::E => next.x > middle.x,
        //                 Direction::W => next.x < middle.x,
        //                 Direction::S => next.y > middle.y,
        //                 Direction::N => next.y < middle.y,
        //                 _ => unreachable!("Direction not reachable")
        //             };
        //             if !ok {
        //                 println!("Not ok. {} -> {} | {} -> {} -> {}", lhs, rhs, came_from, middle, next);
        //                 continue 'search
        //             }
        //         }  

        //         let area = Self::area(lhs, rhs);
        //         println!("Ok. {} -> {} | {}", lhs, rhs, area);
        //         areas.push((area, i, j));
        //     }
        //     break;
        // }

        // println!("Areas: {:?}", areas);
        2.to_string()
    }
}

impl MovieTheater {
    pub fn find_next_on_path(current: &Coord<usize>) {

    }

    pub fn minmax(lhs: usize, rhs: usize) -> (usize, usize) {
        if lhs > rhs { (rhs, lhs) } else { (lhs, rhs) }
    }

    pub fn udiff(lhs: usize, rhs: usize) -> usize {
        let (min, max) = Self::minmax(lhs, rhs);
        max - min
    }
    pub fn area(lhs: &Coord<usize>, rhs: &Coord<usize>) -> usize {
        let x = Self::udiff(lhs.x, rhs.x) + 1;
        let y = Self::udiff(lhs.y, rhs.y) + 1;
        x * y
    }
}