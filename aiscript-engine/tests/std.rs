mod common;

mod core {
    use crate::common::{bool, num, str};

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

    #[test]
    fn eq() {
        assert_eq!(exe("<: Core:eq(null, null)").unwrap(), bool(true));
        assert_eq!(exe("<: Core:eq(false, false)").unwrap(), bool(true));
        assert_eq!(exe("<: Core:eq(1, 1)").unwrap(), bool(true));
        assert_eq!(exe("<: Core:eq('a', 'a')").unwrap(), bool(true));
        assert_eq!(exe("<: Core:eq(null, 1)").unwrap(), bool(false));
        assert_eq!(exe("<: Core:eq(false, true)").unwrap(), bool(false));
        assert_eq!(exe("<: Core:eq(1, 2)").unwrap(), bool(false));
        assert_eq!(exe("<: Core:eq('a', 'b')").unwrap(), bool(false));
    }

    #[test]
    fn neq() {
        assert_eq!(exe("<: Core:neq(null, null)").unwrap(), bool(false));
        assert_eq!(exe("<: Core:neq(false, false)").unwrap(), bool(false));
        assert_eq!(exe("<: Core:neq(1, 1)").unwrap(), bool(false));
        assert_eq!(exe("<: Core:neq('a', 'a')").unwrap(), bool(false));
        assert_eq!(exe("<: Core:neq(null, 1)").unwrap(), bool(true));
        assert_eq!(exe("<: Core:neq(false, true)").unwrap(), bool(true));
        assert_eq!(exe("<: Core:neq(1, 2)").unwrap(), bool(true));
        assert_eq!(exe("<: Core:neq('a', 'b')").unwrap(), bool(true));
    }

    #[test]
    fn and() {
        assert_eq!(exe("<: Core:and(false, false)").unwrap(), bool(false));
        assert_eq!(exe("<: Core:and(false, true)").unwrap(), bool(false));
        assert_eq!(exe("<: Core:and(true, false)").unwrap(), bool(false));
        assert_eq!(exe("<: Core:and(true, true)").unwrap(), bool(true));

        assert!(exe("<: Core:and(false, 1)").is_err());
        assert!(exe("<: Core:and(1, false)").is_err());
    }

    #[test]
    fn or() {
        assert_eq!(exe("<: Core:or(false, false)").unwrap(), bool(false));
        assert_eq!(exe("<: Core:or(false, true)").unwrap(), bool(true));
        assert_eq!(exe("<: Core:or(true, false)").unwrap(), bool(true));
        assert_eq!(exe("<: Core:or(true, true)").unwrap(), bool(true));
        assert!(exe("<: Core:or(false, 1)").is_err());
        assert!(exe("<: Core:or(1, false)").is_err());
    }

    #[test]
    fn add() {
        assert_eq!(exe("<: Core:add(3, 2)").unwrap(), num(5.0));
        assert!(exe("<: Core:add(false, 1)").is_err());
        assert!(exe("<: Core:add(1, false)").is_err());
    }

    #[test]
    fn sub() {
        assert_eq!(exe("<: Core:sub(3, 2)").unwrap(), num(1.0));
        assert!(exe("<: Core:sub(false, 1)").is_err());
        assert!(exe("<: Core:sub(1, false)").is_err());
    }

    #[test]
    fn mul() {
        assert_eq!(exe("<: Core:mul(3, 2)").unwrap(), num(6.0));
        assert!(exe("<: Core:mul(false, 1)").is_err());
        assert!(exe("<: Core:mul(1, false)").is_err());
    }

    #[test]
    fn pow() {
        assert_eq!(exe("<: Core:pow(3, 2)").unwrap(), num(9.0));
        assert!(exe("<: Core:pow(false, 1)").is_err());
        assert!(exe("<: Core:pow(1, false)").is_err());
    }

    #[test]
    fn div() {
        assert_eq!(exe("<: Core:div(3, 2)").unwrap(), num(1.5));
        assert!(exe("<: Core:div(false, 1)").is_err());
        assert!(exe("<: Core:div(1, false)").is_err());
    }

    #[test]
    fn modulo() {
        assert_eq!(exe("<: Core:mod(3, 2)").unwrap(), num(1.0));
        assert!(exe("<: Core:mod(false, 1)").is_err());
        assert!(exe("<: Core:mod(1, false)").is_err());
    }
}
