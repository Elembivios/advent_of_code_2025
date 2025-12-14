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
        // let grid = EndlessGrid::new(self.red_tiles.clone());
        // println!("Grid: {}", grid); 
        // let center = self.get_center();
        let path = self.construct_path();

        let mut areas: Vec<(usize, usize, usize)> = vec![];
        for (i, lhs) in self.red_tiles.iter().enumerate() {
            'search: for (j, rhs) in self.red_tiles.iter().enumerate().skip(i + 1) {
                let (min_x, max_x) = Self::minmax(lhs.x, rhs.x);
                let (min_y, max_y) = Self::minmax(lhs.y, rhs.y);

                let center = Coord::new((max_x - min_x) / 2, (max_y - min_y) / 2);
                

                let mut on_edge: Vec<(_, _, Direction)> = vec![];
                for (z, middle) in self.red_tiles.iter().enumerate() {
                    if z == i || z == j {
                        continue;
                    }

                    if middle.x < min_x || middle.x > max_x || middle.y < min_y || middle.y > max_y {
                        continue;
                    }

                    if middle.x == min_x {
                        let came_from = if lhs.x == min_x { lhs } else { rhs };                        
                        on_edge.push((middle, came_from, Direction::W));
                    } else if middle.x == max_x {
                        let came_from = if lhs.x == max_x { lhs } else { rhs };                        
                        on_edge.push((middle, came_from, Direction::E));
                    } else if middle.y == min_y {
                        let came_from = if lhs.y == min_y { lhs } else { rhs };                        
                        on_edge.push((middle, came_from, Direction::N));
                    } else if middle.y == max_y {
                        let came_from = if lhs.y == max_y { lhs } else { rhs };                        
                        on_edge.push((middle, came_from, Direction::S));                                
                    } else {
                        println!("In middle. {} -> {} | {}", lhs, rhs, middle);
                        continue 'search;   
                    }
                }

                for (middle, came_from, edge) in on_edge {
                    let next = self.red_tiles.iter()
                        .filter(|other| other.x == middle.x || other.y == middle.y)
                        .filter(|other| !(*other == came_from || *other == middle))
                        .next().unwrap();
                    let ok = match edge {
                        Direction::E => next.x > middle.x,
                        Direction::W => next.x < middle.x,
                        Direction::S => next.y > middle.y,
                        Direction::N => next.y < middle.y,
                        _ => unreachable!("Direction not reachable")
                    };
                    if !ok {
                        println!("Not ok. {} -> {} | {} -> {} -> {}", lhs, rhs, came_from, middle, next);
                        continue 'search
                    }
                }  

                let area = Self::area(lhs, rhs);
                println!("Ok. {} -> {} | {}", lhs, rhs, area);
                areas.push((area, i, j));
            }
            break;
        }

        println!("Areas: {:?}", areas);
        2.to_string()
    }
}

impl MovieTheater {
    pub fn get_center(&self) -> Coord<usize> {
        let mut min_x = usize::MAX;
        let mut max_x = usize::MIN;
        let mut min_y = usize::MAX;
        let mut max_y = usize::MIN;

        for coord in &self.red_tiles {
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

        Coord::new((max_x - min_x) / 2, (max_y - min_y) / 2)

    }

    pub fn construct_path(&self) -> Vec<Coord<usize>> {
        let mut min_x = &Coord::new(usize::MAX, usize::MAX);
        for c in &self.red_tiles {
            if c.x < min_x.x {
                min_x = c
            }
        }
        let mut current = min_x;
        let mut previous: Option<&Coord<usize>> = None;
        let mut path: Vec<Coord<usize>> = vec![];
        while path.len() < self.red_tiles.len() {            
            let next = self.find_next_on_path(previous, current);
            if previous.is_some() {
                path.push(previous.unwrap().clone())
            }
            previous = Some(current);
            current = next;                        
        }
        path
    }

    pub fn find_next_on_path(&self, previous: Option<&Coord<usize>>, current: &Coord<usize>) -> &Coord<usize> {
        let next = self.red_tiles.iter()
            .filter(|other| other.x == current.x || other.y == current.y)
            .filter(|other| !((previous.is_some() && *other == previous.unwrap()) || *other == current))
            .next().unwrap();
        next
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