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
                if Self::check_id_valid(num.to_string()) == false {
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
                if Self::check2(num.to_string()) == false {
                    invalid_ids.push(num);
                }
                
            }            
        }
        invalid_ids.iter().sum::<u128>().to_string()
    }
}

impl GiftShop {
    fn check_id_valid(mut id: String) -> bool {
        let odd = id.len() % 2 == 1;
        if odd {
            return true;
        }        
        let part_len = id.len() / 2;
        let rhs = id.split_off(part_len);
        id != rhs
    }

    fn check2(id: String) -> bool {
        let max_part_len = id.len() / 2;
        'size_loop: for part_size in (1..=max_part_len).rev() {
            if id.len() % part_size != 0 {
                continue;
            }    
        
            let mut current_min = 0;
            let mut current_max = part_size;            
            let mut lhs = &id[current_min..current_max];

            current_min += part_size;
            current_max += part_size; 
            while current_max <= id.len() {                                
                let rhs = &id[current_min..current_max];
                if lhs != rhs {
                    continue 'size_loop
                }
                current_min += part_size;
                current_max += part_size;
                lhs = rhs;
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
                assert_eq!(GiftShop::check_id_valid(i.to_string()), false);
            } else {
                assert_eq!(GiftShop::check_id_valid(i.to_string()), true);
            }            
        }

        for i in 1188511880..=1188511890 {
            if i == 1188511885 {
                assert_eq!(GiftShop::check_id_valid(i.to_string()), false);
            } else {
                assert_eq!(GiftShop::check_id_valid(i.to_string()), true);
            }            
        }
        assert_eq!(GiftShop::check_id_valid("123123".to_owned()), false);
        assert_eq!(GiftShop::check_id_valid("133123".to_owned()), true);

        assert_eq!(GiftShop::check_id_valid("101".to_owned()), true);        
    }

    #[test]
    fn test_num_valid_02() {
        assert_eq!(GiftShop::check2("11".to_owned()), false); 
        for i in 11..=22 {
            if i == 11 || i == 22 {
                assert_eq!(GiftShop::check2(i.to_string()), false);
            } else {
                assert_eq!(GiftShop::check2(i.to_string()), true);
            }            
        }

        for i in 998..=1012 {
            if i == 999 || i == 1010 {
                assert_eq!(GiftShop::check2(i.to_string()), false);
            } else {
                assert_eq!(GiftShop::check2(i.to_string()), true);
            }            
        }

        for i in 1188511880..=1188511890 {
            if i == 1188511885 {
                assert_eq!(GiftShop::check2(i.to_string()), false);
            } else {
                assert_eq!(GiftShop::check2(i.to_string()), true);
            }            
        }
        assert_eq!(GiftShop::check2("123123".to_owned()), false);
        assert_eq!(GiftShop::check2("133123".to_owned()), true);

        assert_eq!(GiftShop::check2("101".to_owned()), true);        
    }
}