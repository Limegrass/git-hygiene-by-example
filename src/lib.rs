/** Super secret business logic function */
pub fn custom_add(a: u128, b: u128) -> u128 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_values() {
        assert_eq!(custom_add(1, 2), 3)
    }

    #[test]
    fn add_big_values() {
        assert_eq!(
            custom_add(18_446_744_073_709_551_615, 1), 18_446_744_073_709_551_616 // adding to u64::MAX
        )
    }

    #[test]
    fn add_float_not_represented_numbers() {
        assert_eq!(custom_add(9007199254740993, 1), 9007199254740994)
    }
}
