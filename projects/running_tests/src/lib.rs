pub fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the values {}", a);
    10
}

fn private_fn(a: i32, b: i32) -> i32 {
    a + b
}

// Run all tests: cargo test
// Run test with pass in the name: cargo test pass
// Run test with fail in the name: cargo test fail

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(4);
        assert_eq!(5, value);
    }

    #[test]
    fn test_private_function() {
        assert_eq!(4, private_fn(2, 2));
    }
}
