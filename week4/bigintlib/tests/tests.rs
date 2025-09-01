use bigintlib::*;

#[test]
fn test_eq() {
    let a = BigInt::from(0 as i64);
    let b = BigInt::from(0 as i64);
    assert_eq!(a, b);

    let a = BigInt::from(-1 as i64);
    let b = BigInt::from(-1 as i64);
    assert_eq!(a, b);

    let a = BigInt::from(1 as i64);
    let b = BigInt::from(1 as i64);
    assert_eq!(a, b);
}

#[test]
fn test_neq() {
    let a = BigInt::from(0 as i64);
    let b = BigInt::from(1 as i64);
    assert_ne!(a, b);

    let a = BigInt::from(0 as i64);
    let b = BigInt::from(-1 as i64);
    assert_ne!(a, b);

    let a = BigInt::from(-1 as i64);
    let b = BigInt::from(1 as i64);
    assert_ne!(a, b);
}

#[test]
fn test_add() {
    let a = BigInt::from(-1 as i64);
    let b = BigInt::from(1 as i64);
    let result = BigInt::from(0 as i64);

    assert_eq!(result, a + b);

    let a = BigInt::from(1 as i64);
    let b = BigInt::from(1 as i64);
    let result = BigInt::from(2 as i64);

    assert_eq!(result, a + b);

    let a = BigInt::from(-1 as i64);
    let b = BigInt::from(-1 as i64);
    let result = BigInt::from(-2 as i64);

    assert_eq!(result, a + b);

    let a = BigInt::from(0 as i64);
    let b = BigInt::from(0 as i64);
    let result = BigInt::from(0 as i64);

    assert_eq!(result, a + b);

    let a = BigInt::from(9 as i64);
    let b = BigInt::from(9 as i64);
    let result = BigInt::from(18 as i64);

    assert_eq!(result, a + b);
}

#[test]
fn add_11_1() {
    let a = BigInt::from(11 as i64);
    let b = BigInt::from(1 as i64);
    let result = BigInt::from(12 as i64);

    assert_eq!(result, a + b);
}

#[test]
fn add_123455_1() {
    let a = BigInt::from(123455 as i64);
    let b = BigInt::from(1 as i64);
    let result = BigInt::from(123456 as i64);

    assert_eq!(result, a + b);
}

#[test]
fn sub_1_1() {
    let a = BigInt::from(1 as i64);
    let b = BigInt::from(1 as i64);
    let result = BigInt::from(0 as i64);

    assert_eq!(&b - &a, &a - &b);
    assert_eq!(result, a - b);
}

#[test]
fn sub_1_m1() {
    let a = BigInt::from(1 as i64);
    let b = BigInt::from(-1 as i64);
    let result = BigInt::from(2 as i64);

    assert_eq!(result, a - b);
}

#[test]
fn test_sub() {
    let a = BigInt::from(-1 as i64);
    let b = BigInt::from(-1 as i64);
    let result = BigInt::from(0 as i64);

    assert_eq!(result, a - b);

    let a = BigInt::from(-1 as i64);
    let b = BigInt::from(1 as i64);
    let result = BigInt::from(-2 as i64);

    assert_eq!(result, a - b);
}

#[test]
fn test_cmp() {
    let one = BigInt::from(1 as i64);
    let minus_one = BigInt::from(-1 as i64);
    let zero = BigInt::from(0 as i64);
    let two_thousand = BigInt::from(2000 as i64);
    let two_thousand_one = BigInt::from(2001 as i64);

    assert_eq!(true, one > minus_one);
    assert_eq!(true, one > zero);
    assert_eq!(true, zero > minus_one);
    assert_eq!(true, two_thousand_one > two_thousand);
}

#[test]
fn test_mul() {
    let a = BigInt::from(1 as i64);
    let b = BigInt::from(1 as i64);
    let result = BigInt::from(1 as i64);

    assert_eq!(result, a * b);

    let a = BigInt::from(1 as i64);
    let b = BigInt::from(2 as i64);
    let result = BigInt::from(2 as i64);

    assert_eq!(result, a * b);

    let a = BigInt::from(1 as i64);
    let b = BigInt::from(0 as i64);
    let result = BigInt::from(0 as i64);

    assert_eq!(result, a * b);

    let a = BigInt::from(0 as i64);
    let b = BigInt::from(1 as i64);
    let result = BigInt::from(0 as i64);

    assert_eq!(result, a * b);

    let a = BigInt::from(1 as i64);
    let b = BigInt::from(100 as i64);
    let result = BigInt::from(100 as i64);

    assert_eq!(result, a * b);

    let a = BigInt::from(1 as i64);
    let b = BigInt::from(123 as i64);
    let result = BigInt::from(123 as i64);

    assert_eq!(result, a * b);

    let a = BigInt::from(2 as i64);
    let b = BigInt::from(123 as i64);
    let result = BigInt::from(246 as i64);

    assert_eq!(result, a * b);

    let a = BigInt::from(1 as i64);
    let b = BigInt::from(-1 as i64);
    let result = BigInt::from(-1 as i64);

    assert_eq!(result, a * b);

    let a = BigInt::from(9 as i64);
    let b = BigInt::from(9 as i64);
    let result = BigInt::from(81 as i64);

    assert_eq!(result, a * b);

    let a = BigInt::from(27 as i64);
    let b = BigInt::from(12345679 as i64);
    let result = BigInt::from(333333333 as i64);

    assert_eq!(result, a * b);
}

#[test]
fn div_1_1() {
    let a = BigInt::from(1 as i64);
    let b = BigInt::from(1 as i64);
    let result = BigInt::from(1 as i64);

    assert_eq!(result, a / b);
}

#[test]
fn div_1_0() {
    let a = BigInt::from(1 as i64);
    let b = BigInt::from(0 as i64);
    let result = BigInt::from(0 as i64);

    assert_eq!(result, a / b);
}

#[test]
fn div_64_8() {
    let a = BigInt::from(64 as i64);
    let b = BigInt::from(8 as i64);
    let result = BigInt::from(8 as i64);

    assert_eq!(result, a / b);
}

#[test]
fn div_1000_25() {
    let a = BigInt::from(100 as i64);
    let b = BigInt::from(25 as i64);
    let result = BigInt::from(4 as i64);

    assert_eq!(result, a / b);
}

#[test]
fn test_lshift() {
    let one = BigInt::from(1 as i64);
    let ten = BigInt::from(10 as i64);
    let hundred = BigInt::from(100 as i64);

    assert_eq!(ten, one.clone() << 1);
    assert_eq!(hundred, one << 2);
}

#[test]
fn greater_50_25() {
    let a = BigInt::from(50 as i64);
    let b = BigInt::from(25 as i64);

    assert_eq!(true, a > b);
    assert_eq!(false, a < b);
}

#[test]
fn from_str_0() {
    let a: BigInt = "0".parse().unwrap();
    let b = BigInt::from(0 as u64);

    assert_eq!(b, a);
}

#[test]
fn from_str_10() {
    let expected = BigInt::from(10 as u64);
    let result: BigInt = "10".parse().unwrap();

    assert_eq!(expected, result);
}

#[test]
fn from_str_12345() {
    let expected = BigInt::from(12345 as u64);
    let result: BigInt = "12345".parse().unwrap();

    assert_eq!(expected, result);
}

#[test]
fn from_str_9999999999999999999999() {
    let expected = BigInt::repeating(9, 22);
    let result: BigInt = "9999999999999999999999".parse().unwrap();

    assert_eq!(expected, result);
}

#[test]
fn from_str_m1() {
    let expected = BigInt::from(-1 as i64);
    let result: BigInt = "-1".parse().unwrap();

    assert_eq!(expected, result);
}

#[test]
fn display_0() {
    let result = &BigInt::from(0 as u64).to_string();
    let expected = "0";

    assert_eq!(expected, result);
}

#[test]
fn display_12345() {
    let result = &BigInt::from(12345 as u64).to_string();
    let expected = "12345";

    assert_eq!(expected, result);
}

#[test]
fn display_m123() {
    let result = &BigInt::from(-123 as i64).to_string();
    let expected = "-123";

    assert_eq!(expected, result);
}

#[test]
fn shr_1() {
    let result = BigInt::from(10 as u64) >> 1;
    let expected = BigInt::from(1 as u64);

    assert_eq!(expected, result);
}

#[test]
fn pow_1_1() {
    let a = BigInt::from(1 as u64);
    let expected = BigInt::from(1 as u64);

    assert_eq!(expected, a.pow(1));
}

#[test]
fn pow_1_2() {
    let a = BigInt::from(1 as u64);
    let expected = BigInt::from(1 as u64);

    assert_eq!(expected, a.pow(2));
}

#[test]
fn pow_2_2() {
    let a = BigInt::from(2 as u64);
    let expected = BigInt::from(4 as u64);

    assert_eq!(expected, a.pow(2));
}

#[test]
fn pow_2_4() {
    let a = BigInt::from(2 as u64);
    let expected = BigInt::from(16 as u64);

    assert_eq!(expected, a.pow(4));
}

#[test]
fn pow_10_2() {
    let a = BigInt::from(10 as u64);
    let expected = BigInt::from(100 as u64);

    assert_eq!(expected, a.pow(2));
}
