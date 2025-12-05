use std::cmp;

use crate::utils::digits_iterator::digits;

pub struct GiftShop {
    ranges: Vec<(usize, usize)>
}

impl crate::Advent for GiftShop {
    fn new(data: &str) -> Self
        where 
            Self: Sized {

        let line: &str = data.lines().next().unwrap();
        let ranges = line
            .split(",")
            .map(|p| {
                let v: Vec<usize> = p.split("-").map(|id| id.parse::<usize>().unwrap()).collect();
                (v[0], v[1])            
            }).collect();
        Self { ranges: ranges }
    }

    fn part_01(&self) -> String {
        let mut invalid_ids: Vec<usize> = vec![];
        for range in &self.ranges {
            for num in range.0..=range.1 {
                if Self::check_id_valid_01(num.to_string()) == false {
                    invalid_ids.push(num);
                }
                
            }            
        }
        invalid_ids.iter().sum::<usize>().to_string()
    }

    fn part_02(&self) -> String {
        let mut invalid_ids: Vec<usize> = vec![];
        for range in &self.ranges {
            for num in range.0..=range.1 {                
                if Self::check_id_valid_02(num.to_string()) == false {
                    invalid_ids.push(num);
                }                
            }            
        }
        invalid_ids.iter().sum::<usize>().to_string()
    }
}

impl GiftShop {
    fn check_id_valid_01(id: String) -> bool {
        let odd = id.len() % 2 == 1;
        if odd {
            return true;
        }        

        let part_len = id.len() / 2;
        let lhs = &id[0..part_len];
        let rhs = &id[part_len..];        
        lhs != rhs
    }

    fn check_id_valid_02(id: String) -> bool { 
        'part_length_loop: for part_len in 1..=id.len() / 2 {
            if (id.len() % part_len) != 0 {
                continue;
            }
            let mut i = 0;
            let pattern = &id[i..part_len];
            i += part_len;                        
            while i + part_len <= id.len() {
                let next_pattern = &id[i..i + part_len];
                if pattern != next_pattern {
                    continue 'part_length_loop
                }
                i += part_len;
            }
            return false;
        }
        true
    }

    #[allow(dead_code)]
    fn get_prefix_patterns(start: usize, end: usize) -> (usize, Vec<Vec<usize>>) {
        let start_digits = digits(start).collect();
        let end_digits = digits(end).collect();

        let common_digits = Self::get_common_digits(&start_digits, &end_digits);
        let mut patterns: Vec<Vec<usize>> = vec![];

        let max_len = cmp::min(end_digits.len() / 2, common_digits.len());
        // let max_len = end_digits.len() / 2;
        let mut max_pattern_len = 0;

        'pattern_len: for pattern_len in 1..=max_len {
            let mut i = 0;
            let pattern = &common_digits[i..i + pattern_len];
            i += pattern_len;
            while i + pattern_len <= end_digits.len() {
                let max_index = cmp::min(i + pattern_len, common_digits.len());

                let next_pattern = &common_digits[i..max_index];
                println!("Next pattern: {:?}, max_index: {}", next_pattern, max_index);
                if next_pattern.iter().zip(pattern).any(|(lhs, rhs)| lhs != rhs) {
                    continue 'pattern_len
                }
                // if pattern != next_pattern {
                //     continue 'pattern_len
                // }
                i += pattern_len;
            }
            max_pattern_len = pattern_len;
            patterns.push(pattern.to_vec());
        }
        (max_pattern_len, patterns)
    }

    #[allow(dead_code)]
    fn vec_to_num(v: Vec<usize>) -> usize {
        let mut num = 0;
        for (i, d) in v.iter().enumerate().rev() {
            num += 10usize.pow(i as u32) * d;
        }
        num
    }

    #[allow(dead_code)]
    fn get_common_digits(start: &Vec<usize>, end: &Vec<usize>) -> Vec<usize> {
        let mut common_digits: Vec<_> = vec![];
        for (rhs, lhs) in start.iter().zip(end) {
            if *rhs != *lhs {
                break;
            }
            common_digits.push(*lhs);
        }
        common_digits
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prefix_patterns() {
        let patterns = GiftShop::get_prefix_patterns(1188511880, 1188511890);
        println!("Patterns: {:?}", patterns);
        let patterns = GiftShop::get_prefix_patterns(998, 1012);
        println!("Patterns: {:?}", patterns);
        let patterns = GiftShop::get_prefix_patterns(38593856, 38593862);
        println!("Patterns: {:?}", patterns);
        let patterns = GiftShop::get_prefix_patterns(2121212118, 2121212121);
        println!("Patterns: {:?}", patterns);
        let patterns = GiftShop::get_prefix_patterns(6328350434, 6328506208);
        println!("Patterns: {:?}", patterns);

        for i in 6328350434usize..=6328506208 {
            if !GiftShop::check_id_valid_02(i.to_string())  {
                println!("I --> {}", i);
            }
        }
    }
    #[test]
    fn test_common_digits() {
        let lhs = digits(1188511880).collect();
        let rhs = digits(1188511890).collect();
        let v = GiftShop::get_common_digits(&lhs, &rhs);
        assert_eq!(v, vec![1,1,8,8,5,1,1,8]);
    }

    #[test]
    fn test_num_valid_01() {
        for i in 11..=22 {
            if i == 11 || i == 22 {
                assert_eq!(GiftShop::check_id_valid_01(i.to_string()), false);
            } else {
                assert_eq!(GiftShop::check_id_valid_01(i.to_string()), true);
            }            
        }

        for i in 1188511880..=1188511890 {
            if i == 1188511885 {
                assert_eq!(GiftShop::check_id_valid_01(i.to_string()), false);
            } else {
                assert_eq!(GiftShop::check_id_valid_01(i.to_string()), true);
            }            
        }
        assert_eq!(GiftShop::check_id_valid_01("123123".to_owned()), false);
        assert_eq!(GiftShop::check_id_valid_01("133123".to_owned()), true);

        assert_eq!(GiftShop::check_id_valid_01("101".to_owned()), true);        
    }

    #[test]
    fn test_num_valid_02() {
        assert_eq!(GiftShop::check_id_valid_02("11".to_owned()), false);
        assert_eq!(GiftShop::check_id_valid_02("12".to_owned()), true);
        assert_eq!(GiftShop::check_id_valid_02("101".to_owned()), true);
    }

    #[test] 
    fn test_range_valid_02() {
        for i in 11..=22 {
            let expected = ![11,22].contains(&i);
            assert_eq!(GiftShop::check_id_valid_02(i.to_string()), expected);  
        }
        for i in 99..=115 {
            let expected = ![99,111].contains(&i);
            assert_eq!(GiftShop::check_id_valid_02(i.to_string()), expected);  
        }
        for i in 998..=1012 {
            let expected = ![999,1010].contains(&i);
            assert_eq!(GiftShop::check_id_valid_02(i.to_string()), expected);  
        }
        for i in 1188511880..=1188511890 {
            let expected = ![1188511885].contains(&i);
            assert_eq!(GiftShop::check_id_valid_02(i.to_string()), expected);  
        }
        for i in 222220..=222224 {
            let expected = ![222222].contains(&i);
            assert_eq!(GiftShop::check_id_valid_02(i.to_string()), expected);  
        }
        for i in 1698522..=1698528 {
            let expected = ![].contains(&i);
            assert_eq!(GiftShop::check_id_valid_02(i.to_string()), expected);  
        }
        for i in 446443..=446449 {
            let expected = ![446446].contains(&i);
            assert_eq!(GiftShop::check_id_valid_02(i.to_string()), expected);  
        }
        for i in 38593856..=38593862 {
            let expected = ![38593859].contains(&i);
            assert_eq!(GiftShop::check_id_valid_02(i.to_string()), expected);  
        }
        for i in 565653..=565659 {
            let expected = ![565656].contains(&i);
            assert_eq!(GiftShop::check_id_valid_02(i.to_string()), expected);  
        }
        for i in 824824821..=824824827 {
            let expected = ![824824824].contains(&i);
            assert_eq!(GiftShop::check_id_valid_02(i.to_string()), expected);  
        }
        for i in 2121212118..=2121212124 {
            let expected = ![2121212121].contains(&i);
            assert_eq!(GiftShop::check_id_valid_02(i.to_string()), expected, "at number {}", i);  
        }
    }
}