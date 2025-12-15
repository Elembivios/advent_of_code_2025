
pub struct Factory {
    machines: Vec<Machine>
}

impl crate::Advent for Factory {
    fn new(data: &str) -> Self
        where 
            Self: Sized {
        let machines: Vec<_> = data.lines().map(|l| {
            let mut lights = 0;
            let mut lights_len = 0;
            let mut buttons = vec![];
            let mut joltages = vec![];
            for part in l.split(" ") {
                if part.starts_with('[') {
                    for (i, c) in part.chars().skip(1).enumerate().take(part.len() - 2) {
                        if c == '#' {
                            lights |= 1 << i;
                            lights_len = i;
                        }
                    }
                } else if part.starts_with('(') {                    
                    let p = part[1..part.len() - 1].split(",").map(|c| c.parse::<usize>().unwrap()).collect::<Vec<usize>>();
                    let mut bp: usize = 0;
                    for bi in p {
                        bp |= 1 << bi;
                    }
                    buttons.push(bp);
                } else if part.starts_with('{') {
                    joltages = part[1..part.len() - 1].split(',').map(|c| c.parse::<usize>().unwrap()).collect();
                }                
            }

            println!("Lights: {}", lights);

            Machine {
                lights,
                lights_len,
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
                let new_val = machine.step(0, 0, button.clone(), min);
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
        for machine in &self.machines {
            let joltage_counters: Vec<usize> = vec![0; machine.lights_len];
            
        }
        2.to_string()
    }
}

pub struct Machine {
    lights: usize,
    lights_len: usize,
    buttons: Vec<usize>,
    joltages: Vec<usize>
}

impl Machine {
    pub fn step(&self, lights: usize, counter: usize, button: usize, previous_min: usize) -> usize {        
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
        
        let mut min_counter = previous_min;
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

    pub fn switch(lights: &mut usize, bw: &usize) {
        *lights ^= bw;
    }
}