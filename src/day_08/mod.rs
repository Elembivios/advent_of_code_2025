pub struct Playground {
    junkctions: Vec<[isize; 3]>,
    distances: Vec<(isize, usize, usize)>,
    to_connect: usize
}

impl crate::Advent for Playground {
    fn new(data: &str) -> Self
        where 
            Self: Sized {
        let junkctions: Vec<[isize; 3]> = data.lines().map(|l| {
            let point: [isize; 3] = l.split(",").map(|s| s.parse::<isize>().unwrap()).collect::<Vec<isize>>().try_into().unwrap();
            point
        }).collect();

        let mut distances: Vec<(isize, usize, usize)> = vec![];
        for (i, lhs) in junkctions.iter().enumerate() {
            for (j, rhs) in junkctions.iter().enumerate().skip(i+1) {      
                let distance = Self::distance(lhs, rhs);            
                distances.push((distance, i, j));
            }
        }
        distances.sort_by_key(|d| d.0);

        let to_connect = if junkctions.len() < 100 {
            10
        } else {
            1000
        };        
        Self { junkctions, distances, to_connect }
    }

    fn part_01(&self) -> String {
        let mut circuts: Vec<Vec<usize>> = vec![];
        let mut counter = 0;
        
        'dist: for (_d, lhs_i, rhs_i) in &self.distances {        
            let lhs_c = circuts.iter().enumerate().filter_map(|(z, c)| {
                if c.contains(lhs_i) { Some(z) } else { None }
            }).next();
            let rhs_c = circuts.iter().enumerate().filter_map(|(z, c)| {
                if c.contains(rhs_i) { Some(z) } else { None }
            }).next();

            match (lhs_c, rhs_c) {
                (Some(lhs_c), Some(rhs_c)) => {                    
                    if lhs_c == rhs_c {
                        counter += 1;
                        continue 'dist;
                    } else {                            
                        let rhs_circut = circuts.get(rhs_c).unwrap().clone();
                        let lhs_circut = circuts.get_mut(lhs_c).unwrap();
                        lhs_circut.extend(rhs_circut);
                        circuts.remove(rhs_c);
                        counter += 1;
                    }
                },
                (Some(lhs_c), None) => {
                    let lhs_circut = circuts.get_mut(lhs_c).unwrap();
                    lhs_circut.push(*rhs_i);
                    counter += 1;
                },
                (None, Some(rhs_c)) => {
                    let rhs_circut = circuts.get_mut(rhs_c).unwrap();
                    rhs_circut.push(*lhs_i);
                    counter += 1;
                },
                (None, None) => {
                    circuts.push(vec![*lhs_i, *rhs_i]);
                    counter += 1;
                }
            }

            if counter >= self.to_connect {
                break;
            }
        }

        let mut circuts_by_size: Vec<usize> = circuts.into_iter().map(|c| c.len()).collect();
        circuts_by_size.sort();

        let mul = circuts_by_size.into_iter().rev().take(3).fold(1, |res, a| res * a);
        mul.to_string()
    }

    fn part_02(&self) -> String {
        let mut circuts: Vec<Vec<usize>> = vec![];
        let mut counter = 0;
        let mut last_connection: Option<([isize; 3], [isize; 3])> = None;
        'dist: for (_d, lhs_i, rhs_i) in &self.distances {        
            let lhs_c = circuts.iter().enumerate().filter_map(|(z, c)| {
                if c.contains(lhs_i) { Some(z) } else { None }
            }).next();
            let rhs_c = circuts.iter().enumerate().filter_map(|(z, c)| {
                if c.contains(rhs_i) { Some(z) } else { None }
            }).next();
            match (lhs_c, rhs_c) {
                (Some(lhs_c), Some(rhs_c)) => {                    
                    if lhs_c == rhs_c {
                        continue 'dist;
                    } else {                            
                        let rhs_circut = circuts.get(rhs_c).unwrap().clone();
                        let lhs_circut = circuts.get_mut(lhs_c).unwrap();
                        lhs_circut.extend(rhs_circut);
                        circuts.remove(rhs_c);
                        counter += 1;
                    }
                },
                (Some(lhs_c), None) => {
                    let lhs_circut = circuts.get_mut(lhs_c).unwrap();
                    lhs_circut.push(*rhs_i);
                    counter += 1;
                },
                (None, Some(rhs_c)) => {
                    let rhs_circut = circuts.get_mut(rhs_c).unwrap();
                    rhs_circut.push(*lhs_i);
                    counter += 1;
                },
                (None, None) => {
                    circuts.push(vec![*lhs_i, *rhs_i]);                    
                    counter += 1;
                }
            }

            if counter >= self.junkctions.len() - 1 {
                last_connection = Some((self.junkctions[*lhs_i], self.junkctions[*rhs_i]));
                println!("{} | {:?} - {:?}", _d, self.junkctions[*lhs_i], self.junkctions[*rhs_i]);
                break;
            }
        }

        match last_connection {
            None => return 0.to_string(),
            Some((lhs, rhs)) => {
                let res = lhs[0] * rhs[0];
                return res.to_string();
            }
        }
    }
}

impl Playground {
    fn distance(lhs: &[isize; 3], rhs: &[isize; 3]) -> isize {
        let distance = (
            (
                (lhs[0] - rhs[0]).pow(2) + 
                (lhs[1] - rhs[1]).pow(2) + 
                (lhs[2] - rhs[2]).pow(2)
            )
        ).isqrt();
        distance
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // 162,817,812 -> 984,92,344 -> 316
    #[test]
    fn test_distance1() {
        let lhs: [isize; 3] = [162,817,812];
        let rhs: [isize; 3] = [57,618,57];
        let d1 = Playground::distance(&lhs, &rhs);
        let d2 = Playground::distance(&rhs, &lhs);
        assert_eq!(d1, d2);

        let lhs: [isize; 3] = [162,817,812];
        let rhs: [isize; 3] = [984,92,344];
        let d1 = Playground::distance(&lhs, &rhs);        
        let d2 = Playground::distance(&rhs, &lhs);
        println!("D1: {:?}, D2: {:?}", d1, d2);
        assert_eq!(d1, d2);
    }

    #[test]
    fn test_distance2() {
        let lhs: [isize; 3] = [162,817,812];
        let p1: [isize; 3] = [984,92,344];
        let p2: [isize; 3] = [425,690,689];
        let d1 = Playground::distance(&lhs, &p1);
        let d2 = Playground::distance(&lhs, &p2);
        println!("D1: {:?}, D2: {:?}", d1, d2);


    }
}