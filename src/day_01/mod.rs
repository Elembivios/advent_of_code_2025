pub struct SecretEntrance {
    rotations: Vec<i32>
}

impl crate::Advent for SecretEntrance {
    fn new(data: &str) -> Self {
        let rotations = data
            .lines()
            .map(|l| {                
                let mut num = l[1..].parse().unwrap();
                if l.starts_with("L") {
                    num *= -1   
                }
                num
            }).collect::<Vec<_>>();
        Self {
            rotations: rotations
        }
    }

    fn part_01(&self) -> String {
        let mut counter = 0;
        let mut pos = 50;
        for rotation in &self.rotations {
            let (new_pos, _) = Self::rotate(pos, counter, *rotation);
            pos = new_pos;
            if new_pos == 0 {
                counter += 1;
            }
        }
        counter.to_string()
    }

    fn part_02(&self) -> String {
        let mut counter = 0;
        let mut pos = 50;
        for rotation in &self.rotations {
            let (new_pos, new_counter) = Self::rotate(pos, counter, *rotation);
            pos = new_pos;
            counter = new_counter;

        }
        counter.to_string()
    }
}

impl SecretEntrance {
    pub fn rotate(pos: i32, counter: i32, rotation: i32) -> (i32, i32) {
        let mut new_counter = counter;
        let times = rotation / 100;
        new_counter += times.abs();
        let remaining = rotation - (100 * times);

        let mut new_pos = pos + remaining;
        if new_pos < 0 {
            if pos > 0 {
                new_counter += 1;
            }
            new_pos += 100;
        } else if new_pos >= 100 {
            new_counter += 1;
            new_pos -= 100;
        } else if new_pos == 0 {
            new_counter += 1;
        }
        // let sign_changes_char = if new_counter != counter {'✓'} else {'×'};
        // println!("{} | {} -> {} | {} - {} = {}", sign_changes_char, pos, new_pos, rotation, remaining, times);
        debug_assert!(new_counter < (counter + 2 + times.abs()), "counter: {}, new_counter: {}, times: {}", counter, new_counter, times);
        debug_assert!(new_pos >= 0, "new_pos: {}", new_pos);
        debug_assert!(new_pos < 100, "new_pos: {}", new_pos);
        (new_pos, new_counter)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotation() {
        assert_eq!(SecretEntrance::rotate(50, 0, -25), (25, 0));
        assert_eq!(SecretEntrance::rotate(50, 0, -50), (0, 1));
        assert_eq!(SecretEntrance::rotate(50, 0, -75), (75, 1));
        assert_eq!(SecretEntrance::rotate(50, 0, -175), (75, 2));
    }
}