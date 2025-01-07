pub fn hello_world() -> String {
    "hello, world!".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(hello_world(), "hello, world!");
    }
}
