/** supper sekrit business logic function */
pub fn custom_add_2025_04_24_16_29_49_573835727(a: f64, b: f64) -> f64 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_values() {
        assert_eq!(custom_add_2025_04_24_16_29_49_573835727(1.0, 2.0), 3.0)
    }

    #[test]
    fn add_big_values() {
        assert_eq!(
            custom_add_2025_04_24_16_29_49_573835727(18_446_744_073_709_551_615.0, 1.0), 18_446_744_073_709_551_616.0 // adding to u64::MAX
        )
    }
}
