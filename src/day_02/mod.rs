pub struct GiftShop {
    ranges: Vec<(u128, u128)>
}

impl crate::Advent for GiftShop {
    fn new(data: &str) -> Self
        where 
            Self: Sized {

        let line: &str = data.lines().next().unwrap();
        let ranges = line
            .split(",")
            .map(|p| {
                let v: Vec<u128> = p.split("-").map(|id| id.parse::<u128>().unwrap()).collect();
                (v[0], v[1])            
            }).collect();
        Self { ranges: ranges }
    }

    fn part_01(&self) -> String {
        let mut invalid_ids: Vec<u128> = vec![];
        for range in &self.ranges {
            for num in range.0..=range.1 {
                if Self::check_id_valid_01(num.to_string()) == false {
                    invalid_ids.push(num);
                }
                
            }            
        }
        invalid_ids.iter().sum::<u128>().to_string()
    }

    fn part_02(&self) -> String {
        let mut invalid_ids: Vec<u128> = vec![];
        for range in &self.ranges {
            for num in range.0..=range.1 {                
                if Self::check_id_valid_02(num.to_string()) == false {
                    invalid_ids.push(num);
                }                
            }            
        }
        invalid_ids.iter().sum::<u128>().to_string()
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

}

#[cfg(test)]
mod tests {
    use super::*;

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