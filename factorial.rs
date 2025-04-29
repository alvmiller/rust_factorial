// cargo run
// cargo test

pub fn factorial(number: u64) -> u64 {
    if number == 0 || number == 1 {
        1
    } else {
        (2..=number).product()
    }
}

pub fn factorial_recursive(n: u64) -> u64 {
    if n == 0 || n == 1 {
        1
    } else {
        n * factorial_recursive(n - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
    }

    #[test]
    fn test_factorial_recursive() {
        assert_eq!(factorial_recursive(0), 1);
        assert_eq!(factorial_recursive(1), 1);
        assert_eq!(factorial_recursive(5), 120);
    }
}

fn main() {
    println!("factorial(5) = {}", factorial(5));
    println!("factorial_recursive(5) = {}", factorial(5));
}
