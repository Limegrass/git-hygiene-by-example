fn custom_add(a: i64, b: i64) -> i64 {
    a + b
}

fn main() {
    let total = custom_add(1, 1);
    println!("Your total is {total}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_values() {
        assert_eq!(custom_add(1, 2), 3)
    }
}
