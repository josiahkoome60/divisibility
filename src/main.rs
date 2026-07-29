fn main() {
    let number: &str = "2139994938488080438792949749375948937497969792389348394839";
    if is_divisible_by_3(number) {
        println!("{} is divisible by three.", number);
    } else {
        println!("{} is not divisible by three.", number)
    }
}

pub fn verify_input(number: &str) -> bool {
    for character in number.chars() {
        if !character.is_ascii_digit() {
            return false
        }
        continue;
    } 
    return true
}

pub fn sum_of_digits(digits: &str) -> i32 {
    let mut sum: i32 = 0;

    for digit in digits.bytes() {
        let numerical_value: i32 = (digit - b'0') as i32;
        sum += numerical_value;
    }

    sum
}

pub fn is_divisible_by_3(number: &str) -> bool {

    assert!(verify_input(number));

    let sum = sum_of_digits(number);
    
    if sum == 3 || sum == 6 || sum == 9 {
        return true;
    } else if sum > 9 {
        let num = sum.to_string();
        return is_divisible_by_3(&num);
    } else {
        false
    }
}
