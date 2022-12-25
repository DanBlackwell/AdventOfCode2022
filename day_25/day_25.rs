use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;

#[derive(Debug, Clone, Copy)]
enum SnafuUnit {
    Minus_2,
    Minus_1,
    Zero,
    One,
    Two
}

impl SnafuUnit {
    fn to_char(&self) -> char {
        match self {
            SnafuUnit::Minus_2 => '=', 
            SnafuUnit::Minus_1 => '-', 
            SnafuUnit::Zero => '0', 
            SnafuUnit::One => '1', 
            SnafuUnit::Two => '2'
        }
    }

    fn from_char(val: char) -> SnafuUnit {
        match val {
            '=' => SnafuUnit::Minus_2,
            '-' => SnafuUnit::Minus_1,
            '0' => SnafuUnit::Zero,
            '1' => SnafuUnit::One,
            '2' => SnafuUnit::Two,
             _  => panic!(""),
        }
    }

    fn to_i64(&self) -> i64 {
        match self {
            SnafuUnit::Minus_2 => -2, 
            SnafuUnit::Minus_1 => -1, 
            SnafuUnit::Zero => 0, 
            SnafuUnit::One => 1, 
            SnafuUnit::Two => 2
        }
    }

    fn from_i64(val: i64) -> SnafuUnit {
        match val {
            -2 => SnafuUnit::Minus_2,
            -1 => SnafuUnit::Minus_1,
            0 => SnafuUnit::Zero,
            1 => SnafuUnit::One,
            2 => SnafuUnit::Two,
             _  => panic!(""),
        }
    }
}

#[derive(Debug, Clone)]
struct SnafuValue {
    digits: Vec<SnafuUnit>
}

impl SnafuValue {
    fn to_string(&self) -> String {
        self.digits.iter().fold(String::from(""), |mut st, x| { st.push(x.to_char()); return st; })
    }

    fn from_string(stri: String) -> SnafuValue {
        SnafuValue { digits: stri.chars().map(|x| SnafuUnit::from_char(x)).collect::<Vec<SnafuUnit>>() }
    }

    fn to_i64(&self) -> i64 {
        let mut total = 0;
        for (index, digit) in self.digits.iter().rev().enumerate() {
            total += 5_i64.pow(index as u32) * digit.to_i64();
            // println!("index: {index}, digit: {:?}, to_i64: {:?}, power: {:?} total: {total}", digit, digit.to_i64(), 5_i64.pow(index as u32));
        }
        return total
    }

    fn from_i64(val: i64) -> SnafuValue {
        let mut temp = val;
        let mut carry = 0;
        let mut snafu = SnafuValue { digits: Vec::new() };
        // println!("{val} to snafu:", );
        while temp != 0 || carry == 1 {
            // 13 
            // 13 % 5 = 3, 5 + - 2, next_digit
            let mut digit = temp % 5 + carry;
            if digit > 2 { 
                carry = 1; 
                digit = digit - 5; 
            } else {
                carry = 0;
            }
            // println!("  temp: {temp}, digit: {digit}, carry: {carry}, SnafuUnit: {:?}", SnafuUnit::from_i64(digit));
            snafu.digits.push(SnafuUnit::from_i64(digit));
            temp = temp / 5;
        }
        snafu.digits = snafu.digits.into_iter().rev().collect::<Vec<SnafuUnit>>();
        // println!("{:?}", snafu.digits);

        return snafu;
    }
}

fn main() {
    let mut snafu_values = Vec::new();

    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                if line.len() == 0 { continue; }
                
                snafu_values.push(SnafuValue::from_string(line));
            }
        }
    }

    // let i64_values = snafu_values.iter().map(|x| x.to_i64()).collect::<Vec<i64>>();
    let total = snafu_values.iter().fold(0, |acc,x| acc + x.to_i64());
    println!("{}, in snafu: {}", total, SnafuValue::from_i64(total).to_string());
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
