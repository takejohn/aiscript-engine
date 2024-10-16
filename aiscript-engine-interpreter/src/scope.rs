use std::collections::HashMap;

use aiscript_engine_common::Utf16Str;
use utf16_literal::utf16;

use crate::variable::Variable;

type LayeredStates<'gc> = Vec<HashMap<&'gc Utf16Str, Variable<'gc>>>;

pub struct Scope<'gc> {
    parent: Option<&'gc Scope<'gc>>,
    layered_states: LayeredStates<'gc>,
    name: &'gc Utf16Str,
    ns_name: Option<&'gc Utf16Str>,
}

impl<'gc> Scope<'gc> {
    pub fn new(
        layered_states: LayeredStates<'gc>,
        parent: Option<&'gc Scope<'gc>>,
        name: Option<&'gc Utf16Str>,
        ns_name: Option<&'gc Utf16Str>,
    ) -> Self {
        let name = name.unwrap_or_else(|| {
            if layered_states.len() == 1 {
                Utf16Str::new(&utf16!("<root>"))
            } else {
                Utf16Str::new(&utf16!("<anonymous>"))
            }
        });
        Scope {
            parent,
            layered_states,
            name,
            ns_name,
        }
    }
}
