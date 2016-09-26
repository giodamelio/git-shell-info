pub fn hello() -> &'static str {
    "world"
}

#[cfg(test)]
mod tests {
    use super::hello;

    #[test]
    fn it_works() {
        assert_eq!("aa", hello());
    }
}
