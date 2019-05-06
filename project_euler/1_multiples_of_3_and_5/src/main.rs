fn main() {
    let upper_limit = 999;
    let s: u32;
    let s1 = sum_of_divisible_by(3, upper_limit); 
    let s2 = sum_of_divisible_by(5, upper_limit); 
    let s3 = sum_of_divisible_by(15, upper_limit);
    s = s1 + s2 - s3;
    println!("{}", s);
}

fn sum_of_divisible_by(n: u32, upper_limit: u32) -> u32 {
    if n == 0 {
        return 0
    }
    let remainder = upper_limit % n;
    let quotient = upper_limit / n;
    let last_num = upper_limit - remainder;
    (n + last_num)*quotient / 2
}

#[allow(dead_code)]
fn sum(start: u32, end: u32, divisor: u32) -> u32 {
    let mut sum = 0;
    for i in start..end {
        if i % divisor == 0 {
            sum += i;
        }
    }
    sum
}
