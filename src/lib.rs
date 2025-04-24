/** Super secret business logic function */
pub fn custom_add(a: f64, b: f64) -> f64 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_values() {
        assert_eq!(custom_add(1.0, 2.0), 3.0)
    }

    #[test]
    fn add_big_values() {
        assert_eq!(
            custom_add(18_446_744_073_709_551_615.0, 1.0), 18_446_744_073_709_551_616.0 // adding to u64::MAX
        )
    }
}
