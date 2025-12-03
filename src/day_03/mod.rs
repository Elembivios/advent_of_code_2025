pub struct Lobby {
    banks: Vec<Vec<u8>>
}

impl crate::Advent for Lobby {
    fn new(data: &str) -> Self
        where 
            Self: Sized {
        
        let banks: Vec<Vec<u8>> = data.lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect()).collect();
        Self { banks }
    }

    fn part_01(&self) -> String {
        let mut results: Vec<u64> = vec![];
        for bank in &self.banks {
            let max_joltage = Self::get_largest_overload_joltage(bank, 2);
            results.push(max_joltage);
        }
        results.into_iter().fold(0u64, | sum, d| sum + (d as u64)).to_string()
    }

    fn part_02(&self) -> String {        
        let mut results: Vec<u64> = vec![];
        for bank in &self.banks {
            let max_joltage = Self::get_largest_overload_joltage(bank, 12);
            results.push(max_joltage);
        }
        results.into_iter().fold(0u64, | sum, d| sum + (d as u64)).to_string()
    }
}

impl Lobby {
    fn get_largest_overload_joltage(bank: &Vec<u8>, batteries_count: usize) -> u64 {
        let mut batteries: Vec<u8> = vec![];
        let mut current_max = 0;
        let mut next_index = 0;
        let mut remaining_batteries = batteries_count;

        for _ in 0..remaining_batteries {
            for (i, digit) in bank.iter().enumerate().skip(next_index) {
                if i + remaining_batteries > bank.len() {
                    continue;
                }
                if *digit > current_max {
                    next_index = i + 1;
                    current_max = *digit;  
                }
            }
            batteries.push(current_max);
            current_max = 0;
            remaining_batteries -= 1;
        }    

        let mut num: u64 = 0;
        for (i, battery) in batteries.iter().enumerate() {
            let power = batteries.len() - i -1;
            let mul = 10u64.pow(power as u32);
            num += *battery as u64 * mul;
        }
        num
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn s_to_vec(s: &str) -> Vec<u8> {
        s.chars().map(|c| c.to_digit(10).unwrap() as u8).collect()
    }

    #[test]
    fn test_get_jolts() {
        assert_eq!(Lobby::get_largest_overload_joltage(&s_to_vec("987654321111111"), 2), 98);
        assert_eq!(Lobby::get_largest_overload_joltage(&s_to_vec("811111111111119"), 2), 89);
        assert_eq!(Lobby::get_largest_overload_joltage(&s_to_vec("234234234234278"), 2), 78);
        assert_eq!(Lobby::get_largest_overload_joltage(&s_to_vec("818181911112111"), 2), 92);        
    }

    #[test]
    fn test_overload() {
        assert_eq!(Lobby::get_largest_overload_joltage(&s_to_vec("987654321111111"), 12), 987654321111);
        assert_eq!(Lobby::get_largest_overload_joltage(&s_to_vec("811111111111119"), 12), 811111111119);
        assert_eq!(Lobby::get_largest_overload_joltage(&s_to_vec("234234234234278"), 12), 434234234278);
        assert_eq!(Lobby::get_largest_overload_joltage(&s_to_vec("818181911112111"), 12), 888911112111);        
    }
}