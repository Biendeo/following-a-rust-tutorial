#[cfg(test)]
mod tests {
    use adder::adder::add;

    #[test]
    fn assert_raw() {
        let result = add(2, 2);
        assert!(result == 4);
    }

    #[test]
    fn assert_eq() {
        let result = add(2, 2);
        assert_eq!(result, 4, "The result was not what was expected, it should've been 4 but it was {}", result);
    }

    #[test]
    fn assert_ne() {
        let result = add(2, 2);
        assert_ne!(result, 5);
    }

    #[test]
    #[should_panic(expected = "Panic!")]
    fn assert_should_panic() {
        panic!("Panic!");
    }

    #[test]
    fn assert_using_err() -> Result<(), String> {
        let result = add(2, 2);
        if result == 4 {
            Ok(())
        } else {
            Err(String::from("The result is wrong!"))
        }
    }

    use rstest::*;

    #[rstest]
    #[case(2, 2, 4)]
    #[case(5, 10, 15)]
    #[case(125012, 5347, 130359)]
    fn assert_using_parameters(#[case] left: usize, #[case] right: usize, #[case] expected: usize) {
        let result = add(left, right);
        assert_eq!(expected, result);
    }
}
