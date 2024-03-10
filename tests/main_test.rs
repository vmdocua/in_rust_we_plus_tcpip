// tests/main_test.rs
mod tests {
    #[test]
    fn test_calc() {
        assert_eq!(2 + 2, 4);
        assert_eq!(3 * 3, 9);
    }

    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn test_divide_by_zero() {
        let _s = "";
        let _n = 1 / _s.len();
    }
}