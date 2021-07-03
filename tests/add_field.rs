#[cfg(test)]
mod test {
    use fieldXtender::add_field;

    #[test]
    fn add_field1() {
        #[add_field("pub(crate)", "a", "String")]
        struct Test {}

        let t = Test {a : "Test".to_string()};
        assert_eq!(t.a, "Test");
    }

    #[test]
    fn add_field2() {
        #[add_field("pub", "a", "String")]
        struct Test {}

        let t = Test {a : "Test".to_string()};
        assert_eq!(t.a, "Test");
    }
}