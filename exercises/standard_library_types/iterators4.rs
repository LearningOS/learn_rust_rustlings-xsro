// iterators4.rs

pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
    (1..num + 1).fold(1, |acc, x| acc * x)
}

pub fn factorial_recursion(num: u64) -> u64 {
    if num > 1 {
        factorial(num-1) * num
    }
    else {
        1
    }
}

pub fn factorial_loop(num: u64) -> u64 {
    let mut m=1;
    for i in (1..=num) {
        m*=i
    }
    m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
