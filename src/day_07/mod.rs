use std::collections::{HashMap, HashSet};

use crate::utils::coordinate_system::cartesian::Coord;

pub struct Laboratories {
    splitters: Vec<Coord<usize>>,
    start: Coord<usize>,
    height: usize
}

impl crate::Advent for Laboratories {
    fn new(data: &str) -> Self
        where 
            Self: Sized {

        let start = data.lines().next().unwrap().chars().enumerate().filter(|(_, c)| *c == 'S').map(|(i, _)| Coord::new(i, 0)).next().unwrap();
        let splitters: Vec<Coord<usize>> = data.lines().enumerate().map(|(y, l)| {
            l.chars().enumerate().filter(|(_, c)| *c == '^').map(|(x, _)| {
                Coord::new(x, y)
            }).collect::<Vec<_>>()
        }).flatten().collect();
        let height = data.lines().count();
        Self { splitters, start, height }
    }

    fn part_01(&self) -> String {
        let mut current_beams: HashSet<usize> = HashSet::new();
        let mut counter_splits = 0;
        current_beams.insert(self.start.x);
        for y in 0..self.height {
            let mut new_beams: HashSet<usize> = HashSet::new();
            let splitters_in_line: Vec<Coord<usize>> = self.splitters.iter().filter(|s| s.y == y + 1).cloned().collect();
            for beam_x in current_beams {
                let new_beam = Coord::new(beam_x, y + 1);
                if splitters_in_line.contains(&new_beam) {
                    counter_splits += 1;
                    new_beams.insert(beam_x - 1);
                    new_beams.insert(beam_x + 1);
                } else {
                    new_beams.insert(new_beam.x);
                }
            }
            current_beams = new_beams;
        }
        counter_splits.to_string()
    }

    fn part_02(&self) -> String {
        let mut current_beams: HashMap<usize, usize> = HashMap::new();
        current_beams.insert(self.start.x, 1);
        let mut counter_splits = 1;
        for y in 0..self.height {
            let mut new_beams: HashMap<usize, usize> = HashMap::new();
            let splitters_in_line: Vec<Coord<usize>> = self.splitters.iter().filter(|s| s.y == y +1).cloned().collect();
            for (beam_x, counter) in current_beams {
                let new_beam = Coord::new(beam_x, y + 1);
                if splitters_in_line.contains(&new_beam) {
                    counter_splits += counter;
                    let lhs = new_beams.entry(beam_x - 1).or_insert(0);
                    *lhs += counter;
                    let rhs = new_beams.entry(beam_x + 1).or_insert(0);
                    *rhs += counter;                    
                } else {
                    let new_beam = new_beams.entry(new_beam.x).or_insert(0);
                    *new_beam += counter;
                }
            }
            current_beams = new_beams;
        }
        counter_splits.to_string()
    }
}