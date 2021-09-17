pub fn greeting(name: &str) -> String {
    String::from("Hello!")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting didn't contain name, value was {}",
            result
        );
    }
}