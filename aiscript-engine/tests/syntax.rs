mod common;

mod test_if {
    use crate::common::{exe, str};

    #[test]
    fn test_if() {
        let res = exe(r#"
        var msg = "ai"
		if true {
			msg = "kawaii"
		}
		<: msg
        "#)
        .unwrap();
        assert_eq!(res, str("kawaii"));

        let res = exe(r#"
        var msg = "ai"
		if false {
			msg = "kawaii"
		}
		<: msg
        "#)
        .unwrap();
        assert_eq!(res, str("ai"))
    }

    #[test]
    fn test_else() {
        let res = exe(r#"
		var msg = null
		if true {
			msg = "ai"
		} else {
			msg = "kawaii"
		}
		<: msg
        "#)
        .unwrap();
        assert_eq!(res, str("ai"));

        let res = exe(r#"
		var msg = null
		if false {
			msg = "ai"
		} else {
			msg = "kawaii"
		}
		<: msg
        "#)
        .unwrap();
        assert_eq!(res, str("kawaii"));
    }

    #[test]
    fn test_elif() {
        let res = exe(r#"
        var msg = "bebeyo"
		if false {
			msg = "ai"
		} elif true {
			msg = "kawaii"
		}
		<: msg
        "#)
        .unwrap();
        assert_eq!(res, str("kawaii"));

        let res = exe(r#"
		var msg = "bebeyo"
		if false {
			msg = "ai"
		} elif false {
			msg = "kawaii"
		}
		<: msg
        "#)
        .unwrap();
        assert_eq!(res, str("bebeyo"));
    }

    #[test]
    fn test_if_elif_else() {
        let res = exe(r#"
		var msg = null
		if false {
			msg = "ai"
		} elif true {
			msg = "chan"
		} else {
			msg = "kawaii"
		}
		<: msg
        "#)
        .unwrap();
        assert_eq!(res, str("chan"));

        let res = exe(r#"
		var msg = null
		if false {
			msg = "ai"
		} elif false {
			msg = "chan"
		} else {
			msg = "kawaii"
		}
		<: msg
        "#)
        .unwrap();
        assert_eq!(res, str("kawaii"));
    }

    #[test]
    fn expr() {
        let res = exe(r#"
        <: if true "ai" else "kawaii"
        "#)
        .unwrap();
        assert_eq!(res, str("ai"));

        let res = exe(r#"
        <: if false "ai" else "kawaii"
        "#)
        .unwrap();
        assert_eq!(res, str("kawaii"));
    }

    #[test]
    fn scope() {
        assert!(exe(r#"
        if true let a = 1
        <: a
        "#)
        .is_err());

        assert!(exe(r#"
        if false null elif let a = 1
        <: a
        "#)
        .is_err());

        assert!(exe(r#"
        if false null else let a = 1
        <: a
        "#)
        .is_err());
    }
}
