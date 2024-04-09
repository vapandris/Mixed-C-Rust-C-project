#[cfg(test)]
mod test {

    #[test]
    fn test_add() {
        assert_eq!(2 + 2, 4);
        assert_eq!(mult::mult(2, 5), 10);
        assert_eq!(mult::mult(5, 5), 25);
        assert_eq!(mult::mult(100, 1000), 100000);
    }

    #[test]
    fn test_pow() {
        assert_eq!(unsafe { mult::bindings::poww(2, 2) }, 4);
    }
}