pub struct Cafeteria {
    fresh_ingredients: Vec<(usize, usize)>,
    available_ingredients: Vec<usize>
}

impl crate::Advent for Cafeteria {
    fn new(data: &str) -> Self
        where 
            Self: Sized {
        
        let (fresh_str, available_str) = data.split_once("\r\n\r\n").unwrap();

        let fresh: Vec<(usize, usize)> = fresh_str.lines().map(|l| {
            let (lhs, rhs) = l.split_once("-").unwrap();
            let lhs = lhs.parse::<usize>().unwrap();
            let rhs = rhs.parse::<usize>().unwrap();
            (lhs, rhs)
        }).collect();

        let available: Vec<usize> = available_str.lines().map(|l| {
            l.parse::<usize>().unwrap()
        }).collect();
        
        Self { fresh_ingredients: fresh, available_ingredients: available }
    }

    fn part_01(&self) -> String {
        let mut counter = 0;
        'ingredient_loop: for ingredient in &self.available_ingredients {
            for range in &self.fresh_ingredients {
                if (range.0..=range.1).contains(ingredient)  {
                    counter += 1;
                    continue 'ingredient_loop
                }
            }
        }
        counter.to_string()
    }

    fn part_02(&self) -> String {
        let mut ingredients = self.fresh_ingredients.clone();
        ingredients.sort_by(|lhs, rhs| {
            let lhs_cmp = lhs.0.cmp(&rhs.0);
            if lhs_cmp.is_eq() {
                return lhs.1.cmp(&rhs.1)
            } 
            lhs_cmp
        });

        let mut joined_ranges: Vec<(usize, usize)> = vec![];
        let mut current: (usize, usize) = ingredients[0].clone();
        for range in ingredients.iter().skip(1).peekable() {
            if range.0 > current.1 {
                joined_ranges.push(current);
                current = range.clone();
                continue;                
            }

            if range.1 > current.1 {
                current.1 = range.1;
            }
        }
        joined_ranges.push(current);
        joined_ranges.iter().map(|r| (r.0..=r.1).count()).sum::<usize>().to_string()
    }
}