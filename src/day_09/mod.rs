use crate::utils::coordinate_system::cartesian::{Coord, Axis};

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
        let mut path = self.red_tiles.clone();
        path.push(self.red_tiles.iter().next().unwrap().clone());
        let mut max_area = 0;
        for (i, lhs) in self.red_tiles.iter().enumerate() {
            'search: for (j, rhs) in self.red_tiles.iter().enumerate().skip(i + 1) {
                if lhs.x == rhs.x || lhs.y == rhs.y {
                    continue;
                }
                let (min_x, max_x) = Self::minmax(lhs.x, rhs.x);
                let (min_y, max_y) = Self::minmax(lhs.y, rhs.y);
                            
                for (z, c) in self.red_tiles.iter().enumerate() {
                    if z == i || z == j {
                        continue;
                    }
                    let between_x = c.x > min_x && c.x < max_x;
                    let between_y = c.y > min_y && c.y < min_y;
                    if between_x && between_y {
                        continue 'search;
                    }
                }

                for part in path.windows(2) {
                    let p1 = part[0];
                    let p2 = part[1];
                    if p1 == *lhs || p2 == *lhs || p1 == *rhs || p2 == *rhs {
                        continue;
                    }

                    let (same_axis, diff_axis) = if p1.x == p2.x {
                        (Axis::X, Axis::Y)
                    } else {
                        assert_eq!(p1.y, p2.y);
                        (Axis::Y, Axis::X)
                    };
                                        
                    let other_val = *p1.get(&same_axis);
                    
                    let in_between1 = match same_axis {
                        Axis::X => {
                            min_x <= other_val && other_val <= max_x 
                        },
                        Axis::Y => {
                            min_y <= other_val && other_val <= max_y
                        }
                    };

                    if !in_between1 {
                        continue;
                    }

                    let (minp, maxp) = Self::minmax(*p1.get(&diff_axis), *p2.get(&diff_axis));
                    
                    let crosses = match same_axis {
                        Axis::X => {
                            minp < max_y && maxp > min_y
                        },
                        Axis::Y => {
                            minp < max_y && maxp > min_y
                        }
                    };
                    if crosses {
                        continue 'search;
                    }
                }

                let lhs_corner = Coord::new(lhs.x, rhs.y);
                let rhs_corner = Coord::new(rhs.x, lhs.y);
                if !(Self::coord_in_shape(&lhs_corner, &path) && Self::coord_in_shape(&rhs_corner, &path)) {
                    continue 'search;
                }

                let area = Self::area(lhs, rhs);
                if area > max_area {
                    max_area = area;
                }
            }            
        }
        max_area.to_string()
    }
}

impl MovieTheater {
    pub fn intersects_with_section(start: &Coord<usize>, lhs: Coord<usize>, rhs: Coord<usize>) -> bool{
        if lhs.x == rhs.x {
            // Straight line
            if start.x != lhs.x {
                return false;
            }
            if lhs.y >= start.y || rhs.y >= start.y {
                return true;
            } else {
                return false;
            }
        }
        assert_eq!(lhs.y, rhs.y, "Failed {} - {}", lhs, rhs);

        if lhs.y <= start.y {
            return false;
        }

        let (min_x, max_x) = MovieTheater::minmax(lhs.x, rhs.x);
        if (min_x - 1..=max_x).contains(&start.x) {
            return true;
        }
        false
    }

    pub fn coord_in_shape(c: &Coord<usize>, path: &Vec<Coord<usize>>) -> bool {
        let mut intersect_count = 0;
        for window in path.windows(2) {
            let lhs = window[0];
            let rhs = window[1];
            if MovieTheater::intersects_with_section(c, lhs, rhs) {
                intersect_count += 1;
            }
        }

        if intersect_count % 2 == 0 {
            false
        } else {
            true
        }
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