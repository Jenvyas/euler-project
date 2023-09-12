use std::ops::Mul;

#[derive(Debug, PartialEq)]
enum Sign {
    Pos,
    Neg,
}

#[derive(Debug)]
struct BigInt {
    sign: Sign,
    num_vec: Vec<u32>,
}

impl BigInt {
    fn from_str(num: &str) -> Result<Self, &str> {
        if num.len() == 0 {
            return Err("Num must be longer than 0");
        }
        let sign: Sign;
        let unsigned_num;
        if num.chars().nth(0).unwrap() == '-' {
            sign = Sign::Neg;
            unsigned_num = &num[1..];
            if unsigned_num.len() == 0 {
                return Err("Num cannot be just '-'");
            }
        } else {
            sign = Sign::Pos;
            unsigned_num = num;
        }
        let mut num_vec = Vec::with_capacity(unsigned_num.len());
        for char in unsigned_num.chars().rev() {
            if let Some(digit) = char.to_digit(10) {
                num_vec.push(digit);
            } else {
                return Err("Num must be comprised purely of digits or with a leading -");
            }
        }
        Ok(BigInt { sign, num_vec })
    }

    fn from_i32(mut num: i32) -> Self {
        let sign = if num < 0 {
            num *= -1;
            Sign::Neg
        } else {
            Sign::Pos
        };
        let mut num_vec = Vec::new();
        while num != 0 {
            num_vec.push(num as u32 % 10);
            num /= 10;
        }
        BigInt { sign, num_vec }
    }

    fn carry_over(num_vec: &mut Vec<u32>) {
        for i in 0..num_vec.len() {
            if num_vec[i] > 9 {
                num_vec[i + 1] += num_vec[i] / 10;
                num_vec[i] %= 10;
            }
        }
    }

    fn remove_leading_zeros(num_vec: &mut Vec<u32>) {
        for i in (1..num_vec.len()).rev() {
            if num_vec[i] != 0 {
                break;
            }
            num_vec.pop();
        }
    }
}

impl Mul<BigInt> for BigInt {
    type Output = BigInt;

    fn mul(self, rhs: BigInt) -> Self {
        let mut num_vec: Vec<u32> = vec![0; self.num_vec.len() + rhs.num_vec.len()];
        let sign = match (&self.sign, &rhs.sign) {
            (sign, rhs_sign) if sign == rhs_sign => Sign::Pos,
            _ => Sign::Neg,
        };

        for i in 0..rhs.num_vec.len() {
            for j in 0..self.num_vec.len() {
                num_vec[j + i] += self.num_vec[j] * rhs.num_vec[i];
            }
        }

        BigInt::carry_over(&mut num_vec);
        BigInt::remove_leading_zeros(&mut num_vec);

        BigInt { sign, num_vec }
    }
}

impl std::fmt::Display for BigInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut num_str = String::from("");
        if let Sign::Neg = self.sign {
            num_str += "-";
        }
        for i in self.num_vec.iter().rev() {
            num_str += &(i).to_string();
        }
        write!(f, "{}", num_str)
    }
}

fn main() {
    let mut big = BigInt::from_i32(1);
    for _ in 1..=1000 {
        big = big * BigInt::from_i32(2);
    }
    let big_sum = big.num_vec.iter().fold(0, |sum, num| sum + num);
    println!("{}", big_sum);
}
