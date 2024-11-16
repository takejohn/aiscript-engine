mod common;

mod core {
    use crate::common::{bool, str};

    use super::common::exe;

    #[test]
    fn v() {
        assert_eq!(exe("<: Core:v").unwrap(), str("1.0.0"));
    }

    #[test]
    fn ai() {
        assert_eq!(exe("<: Core:ai").unwrap(), str("kawaii"));
    }

    #[test]
    fn not() {
        assert_eq!(exe("<: Core:not(false)").unwrap(), bool(true));
        assert_eq!(exe("<: Core:not(true)").unwrap(), bool(false));
    }
}
