use aiscript_engine::Value;
use common::exe;

mod common;

#[test]
fn empty() {
    assert!(matches!(exe("").unwrap(), Value::Uninitialized));
}
