use common::{bool, exe, num};

mod common;

mod assign {
    use crate::common::{exe, num};

    #[test]
    fn assign() {
        let res = exe(r#"
            var a = 1
            a = 2
            <: a
        "#)
        .unwrap();
        assert_eq!(res, num(2.0));
    }

    #[test]
    fn add_assign() {
        let res = exe(r#"
            var a = 1
            a += 2
            <: a
        "#)
        .unwrap();
        assert_eq!(res, num(3.0));
    }

    #[test]
    fn sub_assign() {
        let res = exe(r#"
            var a = 1
            a -= 2
            <: a
        "#)
        .unwrap();
        assert_eq!(res, num(-1.0));
    }
}

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

#[test]
fn add() {
    let res = exe("<: 2 + 3").unwrap();
    assert_eq!(res, num(5.0));
}

#[test]
fn sub() {
    let res = exe("<: 2 - 3").unwrap();
    assert_eq!(res, num(-1.0));
}

#[test]
fn and() {
    let res = exe("<: false && false").unwrap();
    assert_eq!(res, bool(false));

    let res = exe("<: false && true").unwrap();
    assert_eq!(res, bool(false));

    let res = exe("<: true && false").unwrap();
    assert_eq!(res, bool(false));

    let res = exe("<: true && true").unwrap();
    assert_eq!(res, bool(true));
}

#[test]
fn or() {
    let res = exe("<: false || false").unwrap();
    assert_eq!(res, bool(false));

    let res = exe("<: false || true").unwrap();
    assert_eq!(res, bool(true));

    let res = exe("<: true || false").unwrap();
    assert_eq!(res, bool(true));

    let res = exe("<: true || true").unwrap();
    assert_eq!(res, bool(true));
}
