fn greet() -> String {
    "Hello from Rust demo!".to_string()
}

fn main() {
    println!("{}", greet());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greet_returns_expected_message() {
        assert_eq!(greet(), "Hello from Rust demo!");
    }
}
