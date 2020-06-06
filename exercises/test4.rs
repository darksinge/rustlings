// test4.rs
// This test covers the sections:
// - Modules
// - Macros

macro_rules! my_macro {
    ($e:expr) => (String::from("Hello ") + &$e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_macro_world() {
        assert_eq!(my_macro!("world!"), "Hello world!");
    }

    #[test]
    fn test_my_macro_goodbye() {
        assert_eq!(my_macro!("goodbye!"), "Hello goodbye!");
    }
}
