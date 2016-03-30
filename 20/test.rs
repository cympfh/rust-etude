#[cfg(test)]
mod test {
    fn gcd(a:i32, b:i32) -> i32 {
        if b == 0 {
            a
        } else {
            gcd(b, a%b)
        }
    }

    #[test]
    fn gcd_positive_test() {
        assert!(gcd(3, 5) == 1);
        assert!(gcd(2, 5) == 1);
        assert!(gcd(3, 21) == 3);
    }

    #[test]
    #[should_panic]
    fn gcd_negative_test() {
        assert!(gcd(1, 1) == 0);
    }

    
}
