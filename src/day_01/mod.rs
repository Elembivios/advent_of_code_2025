// 5956

use anyhow::Context;

pub struct SecretEntrance {
    rotations: Vec<(Rotation, u32)>
}

#[derive(Debug)]
enum Rotation {
    Left,
    Right
}

impl crate::Advent for SecretEntrance {
    fn new(data: &str) -> Self {
        let rotations = data
            .lines()
            .map(|l| {                
                let rotation: Rotation = {
                    if l.starts_with("L") {
                        Rotation::Left
                    } else {
                        Rotation::Right
                    }    
                };
                let num: u32 = l[1..].parse().context(format!("{}", l)).unwrap();
                (rotation, num)                                
            }).collect::<Vec<_>>();
        println!("Data: {:?}, Rotations {:?}", data, rotations);
        Self {
            rotations: rotations
        }
    }

    fn part_01(&self) -> String {
        let mut position: u32 = 50;
        let mut counter: u32 = 0;
        for (rotation, step) in &self.rotations {
            let step = if *step > 99 {
                *step % 100
            } else {
                *step
            };
            match rotation {
                Rotation::Left => {
                    let next_step = if step > position {
                        100 - (step - position)
                    } else {
                        position - step
                    };
                    position = next_step;
                },
                Rotation::Right => {
                    let total = step + position;
                    let next_step = if total > 99 {
                        total - 100
                    } else {
                        total
                    };
                    position = next_step;
                }
            }
            if position == 0 {
                counter += 1;
            }
        }
        counter.to_string()
    }

    fn part_02(&self) -> String {
        2.to_string()
        // let mut position: u32 = 50;
        // let mut counter: u32 = 0;
        
        // for (rotation, step) in &self.rotations {
        //     let step = if *step > 99 {
        //         counter += *step / 100;
        //         println!("{} / 100 = {}", *step, *step / 100);
        //         *step % 100
        //     } else {
        //         *step
        //     };

        //     match rotation {
        //         Rotation::Left => {
        //             let next_step = if step > position {
        //                 counter += 1;                        
        //                 100 - (step - position)
        //             } else {
        //                 position - step
        //             };
        //             println!("L{:?} {:?} -> {:?} | {}", step, position, next_step, counter);
        //             position = next_step;
        //         },
        //         Rotation::Right => {
        //             let total = step + position;
        //             let next_step = if total > 99 {
        //                 counter += 1;
        //                 total - 100
        //             } else {
        //                 total
        //             };
        //             println!("R{:?} {:?} -> {:?} | {}", step, position, next_step, counter);
        //             position = next_step;
        //         }
        //     }

        //     // if position == 0 {
        //     //     counter += 1;
        //     // }
        // }
        // counter.to_string()
    }
}