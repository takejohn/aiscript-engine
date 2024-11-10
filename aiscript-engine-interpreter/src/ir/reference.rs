use std::rc::Rc;

use indexmap::IndexMap;

use super::Register;

pub(super) enum Reference {
    Variable {
        dest: Register,
    },
    Index {
        target: Register,
        index: Register,
    },
    Prop {
        target: Register,
        name: Rc<[u16]>,
    },
    Arr {
        items: Vec<Reference>,
    },
    Obj {
        entries: IndexMap<Rc<[u16]>, Reference>,
    },
}
