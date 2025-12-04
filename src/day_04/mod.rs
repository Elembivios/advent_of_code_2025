use crate::utils::point::Grid;

pub struct PrintingDepartment {
    grid: Grid<char>
}

impl crate::Advent for PrintingDepartment {
    fn new(data: &str) -> Self
        where 
            Self: Sized {
        
        let points: Vec<Vec<char>> = data.lines().map(|l| l.chars().collect()).collect();
        let grid = Grid::new(points);
        Self { grid }
    }

    fn part_01(&self) -> String {
        let counter = self.grid
            .iter_points()
            .filter(|p| *p.value == '@')
            .filter(|p| {
                let adjacent_coords = self.grid.adjacent_coords(&p.coord);
                let num_full = adjacent_coords.iter().filter(|c| *self.grid.get_val(c) == '@').count();
                num_full < 4
            }).count();
        counter.to_string()
    }

    fn part_02(&self) -> String {
        let mut grid = self.grid.clone();
        let mut counter = 0;
        loop {
            let to_remove: Vec<_> =  grid
            .iter_points()
            .filter(|p| *p.value == '@')
            .filter(|p| {
                let adjacent_coords = grid.adjacent_coords(&p.coord);
                let num_full = adjacent_coords.iter().filter(|c| *grid.get_val(c) == '@').count();
                num_full < 4
            })
            .map(|p| p.coord.clone())
            .collect();

            if to_remove.len() == 0 {
                break;
            }
            
            counter += to_remove.len();
            for coord in to_remove {
                *grid.get_val_mut(&coord) = '.';
            }
        }
        counter.to_string()
    }
}