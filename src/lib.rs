/** Super secret business logic function */
pub fn custom_add(a: i64, b: i64) -> i64 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_values() {
        assert_eq!(custom_add(1, 2), 3)
    }
}
