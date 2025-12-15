use owo_colors::OwoColorize;

use crate::utils::wait_user_input;

pub struct Factory {
    machines: Vec<Machine>
}

impl crate::Advent for Factory {
    fn new(data: &str) -> Self
        where 
            Self: Sized {
        let machines: Vec<_> = data.lines().map(|l| {
            let mut lights = vec![];
            let mut buttons = vec![];
            let mut joltages = vec![];
            for part in l.split(" ") {
                if part.starts_with('[') {
                    lights = part.chars().skip(1).take(part.len() - 2).map(|c| {
                        if c == '#' {
                            true
                        } else {
                            false
                        }
                    }).collect();
                } else if part.starts_with('(') {                    
                    let p = part[1..part.len() - 1].split(",").map(|c| c.parse::<usize>().unwrap()).collect::<Vec<usize>>();
                    buttons.push(p);
                } else if part.starts_with('{') {
                    joltages = part[1..part.len() - 1].split(',').map(|c| c.parse::<usize>().unwrap()).collect();
                }                
            }
            Machine {
                lights,
                buttons,
                joltages
            }
        }).collect();
        Self { machines }
    }

    fn part_01(&self) -> String {

        let mut total = 0;
        for (i, machine) in self.machines.iter().enumerate() {
            let mut min = usize::MAX;
            for button in &machine.buttons {
                let lights = (0..machine.lights.len()).map(|_| false).collect();
                let new_val = machine.step(lights, 0, button.clone(), min);
                if new_val < min {
                    min = new_val;
                }
            }            
            println!("{} -> {}", i, min);
            total += min;
        }
        
        total.to_string()
    }

    fn part_02(&self) -> String {
        2.to_string()
    }
}

pub struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<usize>
}

impl Machine {
    pub fn step(&self, lights: Vec<bool>, counter: usize, button: Vec<usize>, previous_min: usize) -> usize {        
        let mut lights = lights;
        let mut counter = counter;
        counter += 1;
        Self::switch(&mut lights, &button);
        if lights == self.lights {
            return counter;
        }
        if counter > previous_min || counter > self.buttons.len() {
            return counter;
        }         
        
        let mut min_counter = usize::MAX;
        for bw in &self.buttons {
            if *bw == button {
                continue;
            }
            let new_counter = self.step(lights.clone(), counter, bw.clone(), min_counter);
            if new_counter < min_counter {
                min_counter = new_counter;
            }
        }

        min_counter
    }

    pub fn switch(lights: &mut Vec<bool>, bw: &Vec<usize>) {
        for b in bw {
            let light = lights.get_mut(*b).unwrap();
            *light = !*light;
        }
    }
}