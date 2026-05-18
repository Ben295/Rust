fn main() {
    println!("{}", greet("world"));
}

fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greet_returns_hello_message() {
        assert_eq!(greet("world"), "Hello, world!");
    }
}
