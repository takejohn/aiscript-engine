mod common;

mod core {
    use super::common::exe;

    #[test]
    fn ai() {
        assert!(exe("Core:ai").is_ok());
    }

    #[test]
    fn not() {
        assert!(exe("Core:not").is_ok())
    }
}
