use aiscript_engine_vm::Value;
use common::exe;

mod common;

#[test]
fn empty() {
    assert!(matches!(exe("").unwrap(), Value::Uninitialized));
}
