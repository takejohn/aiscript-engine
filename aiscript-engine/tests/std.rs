mod common;

mod core {
    use std::rc::Rc;

    use aiscript_engine::{utf16, Value};

    use super::common::exe;

    #[test]
    fn ai() {
        assert_eq!(
            exe("<: Core:ai").unwrap(),
            Value::Str(Rc::from(&utf16!("kawaii") as &[_]))
        );
    }

    #[test]
    fn not() {
        assert_eq!(exe("<: Core:not(false)").unwrap(), Value::Bool(true))
    }
}
