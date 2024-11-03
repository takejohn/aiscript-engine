use common::exe;

mod common;

#[test]
fn core_ai() {
    assert!(exe("Core:ai").is_ok());
}
