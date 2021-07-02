

#[cfg(test)]
mod test {
    use simple_macros::add_field;

    #[test]
    fn add_field() {
        #[add_field("pub", "a", "String")]
        struct Test {}

        let t = Test {a : "Test".to_string()};
        assert_eq!(t.a, "Test");
    }


}