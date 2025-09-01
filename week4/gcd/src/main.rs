use std::{
    cmp::Ordering,
    collections::VecDeque,
    error::Error,
    fmt::Display,
    io::{self, BufRead, repeat},
    iter::repeat_n,
    ops::{Add, Div, Mul, Shl, Sub},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Sign {
    #[default]
    Pos,
    Neg,
}

#[derive(Debug, Clone, Default)]
pub struct BigInt {
    pub digits: VecDeque<u8>,
    pub sign: Sign,
}

impl From<u64> for BigInt {
    fn from(value: u64) -> BigInt {
        let mut r = BigInt {
            digits: VecDeque::new(),
            sign: Sign::Pos,
        };

        let mut n = value;

        loop {
            let digit = n % 10;
            r.digits.push_back(digit as u8);

            if n < 10 {
                break;
            }
            n /= 10;
        }

        r
    }
}

impl From<i64> for BigInt {
    fn from(value: i64) -> BigInt {
        let mut r = BigInt {
            digits: VecDeque::new(),
            sign: if value >= 0 { Sign::Pos } else { Sign::Neg },
        };

        let mut n: u64 = if value >= 0 {
            value as u64
        } else {
            -value as u64
        };

        loop {
            let digit = n % 10;
            r.digits.push_back(digit as u8);

            if n < 10 {
                break;
            }
            n /= 10;
        }

        r
    }
}

#[derive(Debug)]
pub struct ParseBigIntError;

impl Display for ParseBigIntError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error")
    }
}

impl Error for ParseBigIntError {}

impl FromStr for BigInt {
    type Err = ParseBigIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = BigInt::default();

        let mut chars = if s.starts_with("+") {
            let mut chars = s.chars();
            chars.next();
            chars
        } else if s.starts_with("-") {
            result.sign = Sign::Neg;
            let mut chars = s.chars();
            chars.next();
            chars
        } else {
            s.chars()
        };

        while let Some(c) = chars.next() {
            if let Some(digit) = c.to_digit(10) {
                result.digits.push_front(digit as u8);
            } else {
                return Err(ParseBigIntError);
            }
        }

        Ok(result)
    }
}

impl PartialEq for BigInt {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() || self.sign != other.sign {
            false
        } else {
            self.digits
                .iter()
                .zip(other.digits.iter())
                .fold(true, |acc, (a, b)| acc && a == b)
        }
    }
}

impl Eq for BigInt {}

impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BigInt {
    fn cmp(&self, other: &Self) -> Ordering {
        let pos = match (self.sign, other.sign) {
            (Sign::Pos, Sign::Neg) => return Ordering::Greater,
            (Sign::Neg, Sign::Pos) => return Ordering::Less,
            (Sign::Pos, Sign::Pos) => true,
            (Sign::Neg, Sign::Neg) => false,
        };

        if self.len() > other.len() {
            if pos {
                return Ordering::Greater;
            } else {
                return Ordering::Less;
            }
        } else if self.len() < other.len() {
            if pos {
                return Ordering::Less;
            } else {
                return Ordering::Greater;
            }
        }

        self.digits
            .iter()
            .zip(other.digits.iter())
            .rev()
            .fold(Ordering::Equal, |acc, (a, b)| {
                if acc == Ordering::Equal {
                    if a == b {
                        Ordering::Equal
                    } else if a > b {
                        if pos {
                            Ordering::Greater
                        } else {
                            Ordering::Less
                        }
                    } else {
                        if pos {
                            Ordering::Less
                        } else {
                            Ordering::Greater
                        }
                    }
                } else {
                    acc
                }
            })
    }
}

impl Shl<u8> for BigInt {
    type Output = BigInt;

    // multiply self by 10^rhs
    fn shl(self, rhs: u8) -> Self::Output {
        let mut r = self;

        if r.len() == 0 {
            return r;
        }

        if r.len() == 1 && r.digits[0] == 0 {
            return r;
        }

        for _ in 0..rhs {
            r.digits.push_front(0);
        }

        r
    }
}

impl Add<&BigInt> for &BigInt {
    type Output = BigInt;

    fn add(self, rhs: &BigInt) -> Self::Output {
        let mut result = BigInt::default();

        result.sign = match (self.sign, rhs.sign) {
            (a, b) if a == b => a,
            (Sign::Neg, _) => {
                let mut lhs = self.clone();
                lhs.invert();
                return rhs.sub(&lhs);
            }
            (_, Sign::Neg) => {
                let mut rhs = rhs.clone();
                rhs.invert();
                return self.sub(&rhs);
            }

            _ => unreachable!(),
        };

        let (min_len, max_len) = if self.len() > rhs.len() {
            (rhs.len(), self.len())
        } else {
            (self.len(), rhs.len())
        };

        let mut carry: u8 = 0;

        for i in 0..max_len + 1 {
            let sum = match (self.digits.get(i), rhs.digits.get(i)) {
                (Some(lhs), Some(rhs)) => carry + lhs + rhs,
                (Some(lhs), None) => carry + lhs,
                (None, Some(rhs)) => carry + rhs,
                (None, None) => carry,
            };
            result.digits.push_back(sum % 10);
            carry = sum / 10;
        }

        println!("{result:?}");
        result.zero_justify();
        println!("{result:?}");

        result
    }
}

impl Add<BigInt> for BigInt {
    type Output = BigInt;

    fn add(self, rhs: BigInt) -> Self::Output {
        (&self).add(&rhs)
    }
}

impl Sub<&BigInt> for &BigInt {
    type Output = BigInt;

    fn sub(self, rhs: &BigInt) -> Self::Output {
        // Sub function where lhs > 0, rhs > 0
        let sub = |lhs: &BigInt, rhs: &BigInt| -> BigInt {
            let (lhs, rhs) = match lhs.cmp(&rhs) {
                Ordering::Less => (rhs, lhs),
                Ordering::Equal | Ordering::Greater => (lhs, rhs),
            };

            let mut borrow = 0;
            let mut result = BigInt::new();

            for i in 0..lhs.len() {
                let rhs = if i < rhs.len() {
                    rhs.digits[i] as i16
                } else {
                    0
                };

                let digit: i16 = (lhs.digits[i] as i16) - borrow - rhs;
                borrow = if lhs.digits[i] > 0 { 0 } else { borrow };
                let digit = if digit < 0 {
                    borrow = 1;
                    digit + 10
                } else {
                    digit
                };
                result.digits.push_back(digit as u8);
            }

            result.zero_justify();
            result
        };

        match (self.is_negative(), rhs.is_negative()) {
            (true, _) | (_, true) => {
                let mut rhs = rhs.clone();
                rhs.invert();
                self + &rhs
            }
            (false, false) => sub(&self, &rhs),
        }
    }
}

impl Sub<BigInt> for BigInt {
    type Output = BigInt;

    fn sub(self, rhs: BigInt) -> Self::Output {
        (&self).sub(&rhs)
    }
}

impl Mul<&BigInt> for &BigInt {
    type Output = BigInt;

    fn mul(self, rhs: &BigInt) -> Self::Output {
        if self.is_zero() || rhs.is_zero() {
            return BigInt::zero();
        }

        let mut result = BigInt::zero();
        let mut row = self.clone();

        for i in 0..rhs.len() {
            for _ in 0..rhs.digits[i] {
                result = &row + &result;
            }
            row = row << 1;
        }

        result.sign = self.sign_mul(&rhs);
        result.zero_justify();
        result
    }
}

impl Mul<BigInt> for BigInt {
    type Output = BigInt;

    fn mul(self, rhs: BigInt) -> Self::Output {
        (&self).mul(&rhs)
    }
}

impl Div<&BigInt> for &BigInt {
    type Output = BigInt;

    fn div(self, other: &BigInt) -> Self::Output {
        if self.is_zero() || other.is_zero() {
            return BigInt::zero();
        }

        let mut result = BigInt::zero();
        let mut row = result.clone();

        let mut lhs = self.clone();
        lhs.sign = Sign::Pos;
        let mut rhs = other.clone();
        rhs.sign = Sign::Pos;

        for i in (0..lhs.len()).rev() {
            row = row << 1;
            row.digits[0] = lhs.digits[i];

            if result.len() <= i {
                result.digits.push_back(0);
            }

            while row >= rhs {
                println!("before {row:?}, {rhs:?}");
                result.digits[i] += 1;
                row = &row - &rhs;
                println!("after {row:?}, {rhs:?} {}", row >= rhs);
            }
        }

        result.zero_justify();
        result.sign = self.sign_mul(&rhs);
        result
    }
}

impl Div<BigInt> for BigInt {
    type Output = BigInt;

    fn div(self, rhs: BigInt) -> Self::Output {
        (&self).div(&rhs)
    }
}

impl Display for BigInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_negative() {
            write!(f, "-")?;
        }

        for i in (0..self.len()).rev() {
            write!(f, "{}", self.digits[i])?;
        }

        Ok(())
    }
}

impl BigInt {
    pub fn new() -> Self {
        BigInt {
            digits: VecDeque::new(),
            sign: Sign::Pos,
        }
    }

    pub fn zero() -> Self {
        let mut digits = VecDeque::new();
        digits.push_front(0);
        BigInt {
            digits,
            sign: Sign::Pos,
        }
    }

    pub fn is_zero(&self) -> bool {
        self.len() == 1 && self.digits[0] == 0
    }

    pub fn invert(&mut self) {
        self.sign = match self.sign {
            Sign::Pos => Sign::Neg,
            Sign::Neg => Sign::Pos,
        }
    }

    pub fn len(&self) -> usize {
        self.digits.len()
    }

    pub fn last_digit(&self) -> usize {
        self.len() - 1
    }

    pub fn is_positive(&self) -> bool {
        self.sign == Sign::Pos
    }

    pub fn is_negative(&self) -> bool {
        self.sign == Sign::Neg
    }

    /// Removes all leading zeros
    pub fn zero_justify(&mut self) {
        while self.len() > 1 && self.digits[self.last_digit().into()] == 0 {
            self.digits.pop_back();
        }

        if self.len() == 1 && self.digits[0] == 0 {
            self.sign = Sign::Pos;
        }
    }

    pub fn sign_mul(&self, other: &BigInt) -> Sign {
        match (self.sign, other.sign) {
            (a, b) if a == b => Sign::Pos,
            _ => Sign::Neg,
        }
    }

    pub fn repeating(digit: u8, count: usize) -> Self {
        assert!(digit < 10);

        BigInt {
            digits: repeat_n(digit, count).collect(),
            sign: Sign::Pos,
        }
    }

    pub fn i32_max() -> Self {
        BigInt::from(i32::MAX as i64)
    }
}

fn main() {
    let io = io::stdin();
    let mut lock = io.lock();
    let mut input = String::new();

    lock.read_line(&mut input).unwrap();

    let input: Vec<_> = input.split_ascii_whitespace().collect();

    let first: BigInt = input[0].parse().unwrap();
    let second: BigInt = input[1].parse().unwrap();

    if first == second {
        println!("{}", first);
    } else {
        println!("1");
    }
}
