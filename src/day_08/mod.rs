// 1000 -- too low
// 990 -- too low
use num::integer::Roots;

pub struct Playground {
    junkctions: Vec<[isize; 3]>,
    // circuts: Vec<Vec<usize>>,
    to_connect: usize
}

impl crate::Advent for Playground {
    fn new(data: &str) -> Self
        where 
            Self: Sized {
        let junkctions: Vec<[isize; 3]> = data.lines().map(|l| {
            let distances: [isize; 3] = l.split(",").map(|s| s.parse::<isize>().unwrap()).collect::<Vec<isize>>().try_into().unwrap();
            distances
        }).collect();
        // let circuts = junkctions.iter().enumerate().map(|(i, _)| {
        //     vec![i]
        // }).collect();
        let to_connect = if junkctions.len() < 100 {
            10
        } else {
            1000
        };
        println!("To connect: {}", to_connect);
        Self { junkctions, to_connect }
    }

    fn part_01(&self) -> String {
        let mut circuts: Vec<Vec<usize>> = vec![];
        let mut counter = 0;
        let mut distances: Vec<(f64, usize, usize)> = vec![];
        for (i, lhs) in self.junkctions.iter().enumerate() {
            for (j, rhs) in self.junkctions.iter().enumerate() {
                if i == j {
                    continue;
                }
                let distance = Self::distance(lhs, rhs);
                if distances.contains(&(distance, j, i)) {
                    continue;
                }
                distances.push((distance, i, j));
            }
        }
        // distances.sort_unstable_by_key(|d| d.0);
        distances.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        for (d, i, j) in distances.iter().take(20) {
            let lhs = self.junkctions[*i];
            let rhs = self.junkctions[*j];
            println!("{} | {},{},{} -> {},{},{}", d, lhs[0], lhs[1], lhs[2], rhs[0], rhs[1], rhs[2]);
        }
        'dist: for (_d, lhs_i, rhs_i) in &distances {        
            let lhs_c = circuts.iter().enumerate().filter_map(|(z, c)| {
                if c.contains(lhs_i) { Some(z) } else { None }
            }).next();
            let rhs_c = circuts.iter().enumerate().filter_map(|(z, c)| {
                if c.contains(rhs_i) { Some(z) } else { None }
            }).next();

            // println!("{} | {:?} - {:?} | {:?}", counter, lhs_c, lhs_c, circuts);
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

            if counter >= self.to_connect - 1 {
                break;
            }
            // if circuts.iter().map(|c|c.len()).sum::<usize>() >= 10 {
            //     break;
            // }
        }

        println!("Circuts: {:?}", circuts);
        let junkctions: Vec<Vec<[isize; 3]>> = circuts.iter().map(|c| {
            c.iter().map(|i| self.junkctions[*i]).collect::<Vec<[isize;3]>>()
        }).collect();
        println!("Junctions: {:?}", junkctions);


        let mut circuts_by_size: Vec<usize> = circuts.into_iter().map(|c| c.len()).collect();
        circuts_by_size.sort();
        println!("Circuts by size: {:?}", circuts_by_size);
        println!("Circuts len: {}", circuts_by_size.iter().sum::<usize>());
        let mul = circuts_by_size.into_iter().rev().take(3).fold(1, |res, a| res * a);
        mul.to_string()
    }

    fn part_02(&self) -> String {
        2.to_string()
    }
}

impl Playground {
    fn distance(lhs: &[isize; 3], rhs: &[isize; 3]) -> f64 {
        let distance = (((lhs[0] - rhs[0]).pow(2) + (lhs[1] - rhs[1]).pow(2) + (lhs[2] - rhs[2]).pow(2)) as f64).sqrt();
        distance
    }
}