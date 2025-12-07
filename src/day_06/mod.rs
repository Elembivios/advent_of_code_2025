pub struct TrashCompactor {
    numbers: Vec<Vec<usize>>,
    signs: Vec<char>,
    cephalopod_numbers: Vec<Vec<usize>>
}

impl crate::Advent for TrashCompactor {
    fn new(data: &str) -> Self
        where 
            Self: Sized {
        let numbers: Vec<Vec<usize>> = data.lines().enumerate().take_while(|(i, _)| *i < data.lines().count() - 1 ).map(|(_, l)| {
            l.split(" ").filter(|s| !s.is_empty()).map(|s| s.parse::<usize>().unwrap()).collect()
        }).collect();
        let signs: Vec<char>= data.lines().rev().next().unwrap().split(" ").filter(|s| !s.is_empty()).map(|s| s.parse::<char>().unwrap()).collect();
        let lines: Vec<&str> = data.lines().enumerate().take_while(|(i, _)| *i < data.lines().count() - 1 ).map(|(_, l)| l).collect();
        let len = lines.iter().next().unwrap().len();
        let mut cephalopod_numbers: Vec<Vec<usize>> = vec![];
        let mut column_i = 0;
        for i in 0..len {
            let number: Vec<char> = lines.iter().map(|l| {
                l[i..i+1].parse::<char>().unwrap()
            }).collect();
            if number.iter().all(|d| *d == ' ') {
                column_i += 1;
                continue;
            }
            let number: usize = number.into_iter().rev().filter(|d| *d != ' ').enumerate().map(|(row_i, d)| {
                (d.to_digit(10).unwrap() * 10u32.pow(row_i as u32)) as usize
            }).sum();
            
            let column = cephalopod_numbers.get_mut(column_i);
            match column {
                Some(col) => col.push(number),
                None => {
                    let col = vec![number];
                    cephalopod_numbers.push(col);
                }
            }            
        }
        for numbers in cephalopod_numbers.iter_mut() {
            numbers.reverse();
        }
        Self { numbers, signs, cephalopod_numbers }
    }

    fn part_01(&self) -> String {
        let mut results: Vec<usize> = vec![];
        for (i, sign) in self.signs.iter().enumerate() {
            let mut res = if *sign == '+' {
                0
            } else {
                1
            };
            for line in &self.numbers {
                let num = line[i];
                match sign {
                    '+' => {res += num},
                    '*' => {res *= num},
                    _ => unreachable!()
                }
            }
            results.push(res);
            
        }
        results.into_iter().sum::<usize>().to_string()        
    }

    fn part_02(&self) -> String {
        let mut results: Vec<usize> = vec![];
        for (i, sign) in self.signs.iter().enumerate() {
            let mut res = if *sign == '+' {
                0
            } else {
                1
            };
            for num in &self.cephalopod_numbers[i] {
                match sign {
                    '+' => {res += num},
                    '*' => {res *= num},
                    _ => unreachable!()
                }                
            }
            results.push(res);            
        }
        results.into_iter().sum::<usize>().to_string()  
    }
}