#[allow(dead_code)]
pub fn digits(mut num: usize) -> impl Iterator<Item = usize> {
    let mut divisor = 1;
    while num >= divisor * 10 {
        divisor *= 10;
    }

    std::iter::from_fn(move || {
        if divisor == 0 {
            None
        } else {
            let v = num / divisor;
            num %= divisor;
            divisor /= 10;
            Some(v)
        }
    })
}


#[allow(dead_code)]
pub struct DigitsIterator {
    number: u32,
    num_len: u32,
}

#[allow(dead_code)]
impl DigitsIterator {
    fn new(number: u32) -> Self {
        let len = number.checked_ilog10().unwrap_or(0) + 1;
        Self {number, num_len: len}
    }
}

#[allow(dead_code)]
impl Iterator for DigitsIterator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.num_len == 0 {
            return None;
        }

        let divisor = 10u32.pow(self.num_len - 1);
        let first_digit = self.number / divisor;

        self.number = self.number - (first_digit * divisor);
        self.num_len -= 1;

        Some(first_digit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter() {
        let num: u32 = 12345;
        let iterator = DigitsIterator::new(num);
        let digits: Vec<u32> = iterator.collect();
        
        assert_eq!(digits, vec![1,2,3,4,5]);

        let num: u32 = 3216533123;
        let iterator = DigitsIterator::new(num);
        let digits: Vec<u32> = iterator.collect();
        
        assert_eq!(digits, vec![3, 2, 1, 6, 5, 3, 3, 1, 2, 3]);
    }
}
