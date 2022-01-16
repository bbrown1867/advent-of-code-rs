fn hello_world() -> u8 {
    println!("Hello World!");
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hello_world() {
        assert_eq!(hello_world(), 0);
    }
}
